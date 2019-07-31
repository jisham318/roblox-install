use std::{
    fmt, io,
    path::{Path, PathBuf},
};

#[cfg(target_os = "windows")]
use winreg::RegKey;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MalformedRegistry,
    PlatformNotSupported,
    RegistryError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::MalformedRegistry => write!(formatter, "The values of the registry keys used to find Roblox are malformed, maybe your Roblox installation is corrupt?"),
            Error::PlatformNotSupported => write!(formatter, "Your platform is not currently supported"),
            Error::RegistryError(error) => write!(formatter, "Couldn't find registry keys, Roblox might not be installed. ({})", error),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Error::RegistryError(error) = self {
            Some(error)
        } else {
            None
        }
    }
}

#[derive(Debug)]
#[must_use]
pub struct RobloxStudio {
    root: PathBuf,
    plugins: PathBuf,
}

impl RobloxStudio {
    #[cfg(target_os = "windows")]
    pub fn locate() -> Result<RobloxStudio> {
        let hkcu = RegKey::predef(winreg::enums::HKEY_CURRENT_USER);

        let roblox_studio_reg = hkcu
            .open_subkey(r"Software\Roblox\RobloxStudio")
            .map_err(Error::RegistryError)?;

        let content_folder_value: String = roblox_studio_reg
            .get_value("ContentFolder")
            .map_err(Error::RegistryError)?;

        let content_folder_path = PathBuf::from(content_folder_value);

        let root = content_folder_path.parent()
            .ok_or(Error::MalformedRegistry)?;

        let plugins = root.parent()
            .ok_or(Error::MalformedRegistry)?.parent()
            .ok_or(Error::MalformedRegistry)?.join("Plugins");

        Ok(RobloxStudio {
            root: root.to_owned(),
            plugins: plugins.to_owned(),
        })
    }

    #[cfg(not(target_os = "windows"))]
    #[inline]
    pub fn locate() -> Result<RobloxStudio> {
        Err(Error::PlatformNotSupported)
    }

    #[must_use]
    #[inline]
    pub fn root_path(&self) -> &Path {
        &self.root
    }

    #[must_use]
    #[inline]
    pub fn exe_path(&self) -> PathBuf {
        self.root.join("RobloxStudioBeta.exe")
    }

    #[must_use]
    #[inline]
    pub fn built_in_plugins_path(&self) -> PathBuf {
        self.root.join("BuiltInPlugins")
    }

    #[must_use]
    #[inline]
    pub fn plugins_path(&self) -> PathBuf {
        self.plugins.to_owned()
    }
}
