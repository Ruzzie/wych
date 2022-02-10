module AppVersion exposing (current, versionString, VersionInfo)

import Time exposing (..)


current : VersionInfo
current =
    \{ version = "{version}"
    , buildNumber = {build_number}
    , hash = "{hash}"
    , timestamp = millisToPosix {timestamp}
    , source = "{source}"
    }


versionString : String
versionString =
    toString current


type alias VersionInfo =
    \{ version : String
    , buildNumber : Int
    , hash : String
    , timestamp : Posix
    , source : String
    }


toString : VersionInfo -> String
toString versionInfo =
    versionInfo.version ++ " " ++ String.fromInt versionInfo.buildNumber ++ " - [" ++ versionInfo.hash ++ "] - " ++ String.fromInt (toYear utc versionInfo.timestamp) ++ " - (" ++ versionInfo.source ++ ")"