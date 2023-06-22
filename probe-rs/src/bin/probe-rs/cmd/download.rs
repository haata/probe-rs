use std::fs::File;
use std::path::Path;

use anyhow::Context;
use probe_rs::flashing::FileDownloadError;
use probe_rs::flashing::{BinOptions, Format};

use crate::util::common_options::ProbeOptions;
use crate::util::common_options::{CargoOptions, FlashOptions};
use crate::util::flash::run_flash_download;
use crate::util::parse_u32;
use crate::util::parse_u64;

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
enum DownloadFileType {
    Elf,
    Hex,
    Bin,
}

impl DownloadFileType {
    fn into(self, base_address: Option<u64>, skip: Option<u32>) -> Format {
        match self {
            DownloadFileType::Elf => Format::Elf,
            DownloadFileType::Hex => Format::Hex,
            DownloadFileType::Bin => Format::Bin(BinOptions {
                base_address,
                skip: skip.unwrap_or(0),
            }),
        }
    }
}

#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(flatten)]
    common: ProbeOptions,

    /// Format of the file to be downloaded to the flash. Possible values are case-insensitive.
    #[clap(value_enum, ignore_case = true, default_value = "elf", long)]
    format: DownloadFileType,

    /// The address in memory where the binary will be put at. This is only considered when `bin` is selected as the format.
    #[clap(long, value_parser = parse_u64)]
    base_address: Option<u64>,
    /// The number of bytes to skip at the start of the binary file. This is only considered when `bin` is selected as the format.
    #[clap(long, value_parser = parse_u32)]
    skip_bytes: Option<u32>,

    /// The path to the file to be downloaded to the flash
    path: String,

    /// Whether to erase the entire chip before downloading
    #[clap(long)]
    chip_erase: bool,

    /// Whether to disable fancy progress reporting
    #[clap(long)]
    disable_progressbars: bool,

    /// Disable double-buffering when downloading flash.  If downloading times out, try this option.
    #[clap(long = "disable-double-buffering")]
    disable_double_buffering: bool,
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        let mut session = self.common.simple_attach()?;

        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(e) => return Err(FileDownloadError::IO(e)).context("Failed to open binary file."),
        };

        let mut loader = session.target().flash_loader();

        let format = self.format.into(self.base_address, self.skip_bytes);
        match format {
            Format::Bin(options) => loader.load_bin_data(&mut file, options),
            Format::Elf => loader.load_elf_data(&mut file),
            Format::Hex => loader.load_hex_data(&mut file),
        }?;

        run_flash_download(
            &mut session,
            Path::new(&self.path),
            &FlashOptions {
                disable_progressbars: self.disable_progressbars,
                disable_double_buffering: self.disable_double_buffering,
                reset_halt: false,
                log: None,
                restore_unwritten: false,
                flash_layout_output_path: None,
                elf: None,
                work_dir: None,
                cargo_options: CargoOptions::default(),
                probe_options: self.common,
            },
            loader,
            self.chip_erase,
        )?;

        Ok(())
    }
}
