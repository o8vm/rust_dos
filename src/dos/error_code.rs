use core::convert::TryFrom;
use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    Success = 0,
    InvalidFunctionNumber = 1,
    FileNotFound = 2,
    PathNotFound = 3,
    TooManyOpenFiles = 4,
    AccessDenied = 5,
    InvalidHandle = 6,
    MemoryControlBlockDestroyed = 7,
    InsufficientMemory = 8,
    InvalidMemoryBlockAddress = 9,
    InvalidEnvironment = 10,
    InvalidFormat = 11,
    InvalidAccessCode = 12,
    InvalidData = 13,
    // 14 is reserved
    InvalidDrive = 15,
    AttemptedToRemoveCurrentDirectory = 16,
    NotSameDevice = 17,
    NoMoreFiles = 18,
    DiskWriteProtected = 19,
    UnknownUnit = 20,
    DriveNotReady = 21,
    UnknownCommand = 22,
    DataErrorCrcFailed = 23,
    BadRequestStructureLength = 24,
    SeekError = 25,
    UnknownMediaType = 26,
    SectorNotFound = 27,
    PrinterOutOfPaper = 28,
    WriteFault = 29,
    ReadFault = 30,
    GeneralFailure = 31,
    SharingViolation = 32,
    LockViolation = 33,
    InvalidDiskChange = 34,
    FcbUnavailable = 35,
    InvalidSharingBuffer = 36,
    CodePageMismatch = 37, // Reserved under MS-DOS 4.0
    CannotCompleteFileOperation = 38, // Reserved under MS-DOS 4.0
    InsufficientDiskSpace = 39, // Reserved under MS-DOS 4.0
    // 40 to 49 are reserved
    NetworkRequestNotSupported = 50,
    RemoteComputerNotListening = 51,
    DuplicateNameOnNetwork = 52,
    NetworkNameNotFound = 53,
    NetworkBusy = 54,
    NetworkDeviceNoLongerExists = 55,
    NetworkBIOSCommandLimitExceeded = 56,
    NetworkAdapterHardwareError = 57,
    IncorrectResponseFromNetwork = 58,
    UnexpectedNetworkError = 59,
    IncompatibleRemoteAdapter = 60,
    PrintQueueFull = 61,
    NotEnoughSpaceForPrintFile = 62,
    PrintFileDeleted = 63,
    NetworkNameDeleted = 64,
    NetworkAccessDenied = 65,
    NetworkDeviceTypeIncorrect = 66,
    NetworkNameNotFound2 = 67,
    NetworkNameLimitExceeded = 68,
    NetworkBIOSSessionLimitExceeded = 69,
    TemporarilyPaused = 70,
    NetworkRequestNotAccepted = 71,
    NetworkPrintAndDiskRedirectionPaused = 72,
    InvalidNetworkVersion = 73, // Reserved on DOS, used only on LANtastic
    AccountExpired = 74, // Reserved on DOS, used only on LANtastic
    PasswordExpired = 75, // Reserved on DOS, used only on LANtastic
    LoginAttemptedInvalidAtThisTime = 76, // Reserved on DOS, used only on LANtastic
    DiskLimitExceedOnNetworkNode = 77, // Reserved on DOS, used only on LANtastic
    NotLoggedInToNetworkMode = 78, // Reserved on DOS, used only on LANtastic
    // 79 is reserved
    FileAlreadyExists = 80,
    // 81 is reserved
    CannotMakeDirectory = 82,
    FailOnInterrupt24h = 83,
    TooManyRedirections = 84, // Reserved under MS-DOS 3.3
    DuplicateRedirection = 85, // Reserved under MS-DOS 3.3
    InvalidPassword = 86, // Reserved under MS-DOS 3.3
    InvalidParameter = 87, // Reserved under MS-DOS 3.3
    NetworkDeviceFault = 88, // Reserved under MS-DOS 3.3
    FunctionNotSupportedByNetwork = 89, // Reserved under MS-DOS 4.0
    RequiredSystemComponentNotInstalled = 90, // Reserved under MS-DOS 4.0
    UnknownError = 255, // This error doesn't exist in MS-DOS
}

impl ErrorCode {
    pub fn from_u8(value: u8) -> Option<ErrorCode> {
        match value {
            0 => Some(ErrorCode::Success),
            1 => Some(ErrorCode::InvalidFunctionNumber),
            2 => Some(ErrorCode::FileNotFound),
            3 => Some(ErrorCode::PathNotFound),
            4 => Some(ErrorCode::TooManyOpenFiles),
            5 => Some(ErrorCode::AccessDenied),
            6 => Some(ErrorCode::InvalidHandle),
            7 => Some(ErrorCode::MemoryControlBlockDestroyed),
            8 => Some(ErrorCode::InsufficientMemory),
            9 => Some(ErrorCode::InvalidMemoryBlockAddress),
            10 => Some(ErrorCode::InvalidEnvironment),
            11 => Some(ErrorCode::InvalidFormat),
            12 => Some(ErrorCode::InvalidAccessCode),
            13 => Some(ErrorCode::InvalidData),
            15 => Some(ErrorCode::InvalidDrive),
            16 => Some(ErrorCode::AttemptedToRemoveCurrentDirectory),
            17 => Some(ErrorCode::NotSameDevice),
            18 => Some(ErrorCode::NoMoreFiles),
            19 => Some(ErrorCode::DiskWriteProtected),
            20 => Some(ErrorCode::UnknownUnit),
            21 => Some(ErrorCode::DriveNotReady),
            22 => Some(ErrorCode::UnknownCommand),
            23 => Some(ErrorCode::DataErrorCrcFailed),
            24 => Some(ErrorCode::BadRequestStructureLength),
            25 => Some(ErrorCode::SeekError),
            26 => Some(ErrorCode::UnknownMediaType),
            27 => Some(ErrorCode::SectorNotFound),
            28 => Some(ErrorCode::PrinterOutOfPaper),
            29 => Some(ErrorCode::WriteFault),
            30 => Some(ErrorCode::ReadFault),
            31 => Some(ErrorCode::GeneralFailure),
            32 => Some(ErrorCode::SharingViolation),
            33 => Some(ErrorCode::LockViolation),
            34 => Some(ErrorCode::InvalidDiskChange),
            35 => Some(ErrorCode::FcbUnavailable),
            36 => Some(ErrorCode::InvalidSharingBuffer),
            37 => Some(ErrorCode::CodePageMismatch),
            38 => Some(ErrorCode::CannotCompleteFileOperation),
            39 => Some(ErrorCode::InsufficientDiskSpace),
            50 => Some(ErrorCode::NetworkRequestNotSupported),
            51 => Some(ErrorCode::RemoteComputerNotListening),
            52 => Some(ErrorCode::DuplicateNameOnNetwork),
            53 => Some(ErrorCode::NetworkNameNotFound),
            54 => Some(ErrorCode::NetworkBusy),
            55 => Some(ErrorCode::NetworkDeviceNoLongerExists),
            56 => Some(ErrorCode::NetworkBIOSCommandLimitExceeded),
            57 => Some(ErrorCode::NetworkAdapterHardwareError),
            58 => Some(ErrorCode::IncorrectResponseFromNetwork),
            59 => Some(ErrorCode::UnexpectedNetworkError),
            60 => Some(ErrorCode::IncompatibleRemoteAdapter),
            61 => Some(ErrorCode::PrintQueueFull),
            62 => Some(ErrorCode::NotEnoughSpaceForPrintFile),
            63 => Some(ErrorCode::PrintFileDeleted),
            64 => Some(ErrorCode::NetworkNameDeleted),
            65 => Some(ErrorCode::NetworkAccessDenied),
            66 => Some(ErrorCode::NetworkDeviceTypeIncorrect),
            67 => Some(ErrorCode::NetworkNameNotFound2),
            68 => Some(ErrorCode::NetworkNameLimitExceeded),
            69 => Some(ErrorCode::NetworkBIOSSessionLimitExceeded),
            70 => Some(ErrorCode::TemporarilyPaused),
            71 => Some(ErrorCode::NetworkRequestNotAccepted),
            72 => Some(ErrorCode::NetworkPrintAndDiskRedirectionPaused),
            73 => Some(ErrorCode::InvalidNetworkVersion),
            74 => Some(ErrorCode::AccountExpired),
            75 => Some(ErrorCode::PasswordExpired),
            76 => Some(ErrorCode::LoginAttemptedInvalidAtThisTime),
            77 => Some(ErrorCode::DiskLimitExceedOnNetworkNode),
            78 => Some(ErrorCode::NotLoggedInToNetworkMode),
            80 => Some(ErrorCode::FileAlreadyExists),
            82 => Some(ErrorCode::CannotMakeDirectory),
            83 => Some(ErrorCode::FailOnInterrupt24h),
            84 => Some(ErrorCode::TooManyRedirections),
            85 => Some(ErrorCode::DuplicateRedirection),
            86 => Some(ErrorCode::InvalidPassword),
            87 => Some(ErrorCode::InvalidParameter),
            88 => Some(ErrorCode::NetworkDeviceFault),
            89 => Some(ErrorCode::FunctionNotSupportedByNetwork),
            90 => Some(ErrorCode::RequiredSystemComponentNotInstalled),
            255 => Some(ErrorCode::UnknownError),
            _ => None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        return *self as u8;
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::Success => "Success",
            ErrorCode::InvalidFunctionNumber => "Invalid function number",
            ErrorCode::FileNotFound => "File not found",
            ErrorCode::PathNotFound => "Path not found",
            ErrorCode::TooManyOpenFiles => "Too many open files",
            ErrorCode::AccessDenied => "Access denied",
            ErrorCode::InvalidHandle => "Invalid handle",
            ErrorCode::MemoryControlBlockDestroyed => "Memory control block destroyed",
            ErrorCode::InsufficientMemory => "Insufficient memory",
            ErrorCode::InvalidMemoryBlockAddress => "Invalid memory block address",
            ErrorCode::InvalidEnvironment => "Invalid environment",
            ErrorCode::InvalidFormat => "Invalid format",
            ErrorCode::InvalidAccessCode => "Invalid access code",
            ErrorCode::InvalidData => "Invalid data",
            ErrorCode::InvalidDrive => "Invalid drive",
            ErrorCode::AttemptedToRemoveCurrentDirectory => "Attempted to remove current directory",
            ErrorCode::NotSameDevice => "Not same device",
            ErrorCode::NoMoreFiles => "No more files",
            ErrorCode::DiskWriteProtected => "Disk write protected",
            ErrorCode::UnknownUnit => "Unknown unit",
            ErrorCode::DriveNotReady => "Drive not ready",
            ErrorCode::UnknownCommand => "Unknown command",
            ErrorCode::DataErrorCrcFailed => "Data error (CRC failed)",
            ErrorCode::BadRequestStructureLength => "Bad request structure length",
            ErrorCode::SeekError => "Seek error",
            ErrorCode::UnknownMediaType => "Unknown media type",
            ErrorCode::SectorNotFound => "Sector not found",
            ErrorCode::PrinterOutOfPaper => "Printer out of paper",
            ErrorCode::WriteFault => "Write fault",
            ErrorCode::ReadFault => "Read fault",
            ErrorCode::GeneralFailure => "General failure",
            ErrorCode::SharingViolation => "Sharing violation",
            ErrorCode::LockViolation => "Lock violation",
            ErrorCode::InvalidDiskChange => "Invalid disk change",
            ErrorCode::FcbUnavailable => "FCB unavailable",
            ErrorCode::InvalidSharingBuffer => "Invalid sharing buffer",
            ErrorCode::CodePageMismatch => "Code page mismatch",
            ErrorCode::CannotCompleteFileOperation => "Cannot complete file operation",
            ErrorCode::InsufficientDiskSpace => "Insufficient disk space",
            ErrorCode::NetworkRequestNotSupported => "Network request not supported",
            ErrorCode::RemoteComputerNotListening => "Remote computer not listening",
            ErrorCode::DuplicateNameOnNetwork => "Duplicate name on network",
            ErrorCode::NetworkNameNotFound => "Network name not found",
            ErrorCode::NetworkBusy => "Network busy",
            ErrorCode::NetworkDeviceNoLongerExists => "Network device no longer exists",
            ErrorCode::NetworkBIOSCommandLimitExceeded => "Network BIOS command limit exceeded",
            ErrorCode::NetworkAdapterHardwareError => "Network adapter hardware error",
            ErrorCode::IncorrectResponseFromNetwork => "Incorrect response from network",
            ErrorCode::UnexpectedNetworkError => "Unexpected network error",
            ErrorCode::IncompatibleRemoteAdapter => "Incompatible remote adapter",
            ErrorCode::PrintQueueFull => "Print queue full",
            ErrorCode::NotEnoughSpaceForPrintFile => "Not enough space for print file",
            ErrorCode::PrintFileDeleted => "Print file deleted",
            ErrorCode::NetworkNameDeleted => "Network name deleted",
            ErrorCode::NetworkAccessDenied => "Network access denied",
            ErrorCode::NetworkDeviceTypeIncorrect => "Network device type incorrect",
            ErrorCode::NetworkNameNotFound2 => "Network name not found",
            ErrorCode::NetworkNameLimitExceeded => "Network name limit exceeded",
            ErrorCode::NetworkBIOSSessionLimitExceeded => "Network BIOS session limit exceeded",
            ErrorCode::TemporarilyPaused => "Temporarily paused",
            ErrorCode::NetworkRequestNotAccepted => "Network request not accepted",
            ErrorCode::NetworkPrintAndDiskRedirectionPaused => "Network print and disk redirection paused",
            ErrorCode::InvalidNetworkVersion => "Invalid network version",
            ErrorCode::AccountExpired => "Account expired",
            ErrorCode::PasswordExpired => "Password expired",
            ErrorCode::LoginAttemptedInvalidAtThisTime => "Login attempt invalid at this time",
            ErrorCode::DiskLimitExceedOnNetworkNode => "Disk limit exceeded on network node",
            ErrorCode::NotLoggedInToNetworkMode => "Not logged in to network mode",
            ErrorCode::FileAlreadyExists => "File already exists",
            ErrorCode::CannotMakeDirectory => "Cannot make directory",
            ErrorCode::FailOnInterrupt24h => "Fail on interrupt 24h",
            ErrorCode::TooManyRedirections => "Too many redirections",
            ErrorCode::DuplicateRedirection => "Duplicate redirection",
            ErrorCode::InvalidPassword => "Invalid password",
            ErrorCode::InvalidParameter => "Invalid parameter",
            ErrorCode::NetworkDeviceFault => "Network device fault",
            ErrorCode::FunctionNotSupportedByNetwork => "Function not supported by network",
            ErrorCode::RequiredSystemComponentNotInstalled => "Required system component not installed",
            ErrorCode::UnknownError => "Unknown error",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Into<u8> for ErrorCode {
    fn into(self) -> u8 {
        return self.to_u8();
    }
}

impl TryFrom<u8> for ErrorCode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match ErrorCode::from_u8(value) {
            Some(error_code) => Ok(error_code),
            None => Err("Unknown error code"),
        }
    }
}