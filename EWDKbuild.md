# Building using EWDK (Enterprise WDK)

The EWDK is a standalone, self-contained command-line environment for building drivers. It includes Visual Studio Build
Tools, the SDK, and the WDK.

1. Download the [EWDK](https://docs.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk) and mount it
    - It should be mounted as a drive something like `D:` and contain `LaunchBuildEnv.cmd`
2. Now from inside the `fsfilter-rs\minifilter` directory, run the following commands:
    1. From an Administrator command prompt, run `call "D:\LaunchBuildEnv.cmd"`
    2. Followed by `msbuild RWatch.sln /m /p:configuration=Release /p:platform=x64 /p:RunCodeAnalysis=false`

The next steps are the same as [MINIFILTER.md](MINIFILTER.md).

## References

- [download-the-wdk](https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk)
- [using-the-enterprise-wdk](https://learn.microsoft.com/en-us/windows-hardware/drivers/develop/using-the-enterprise-wdk)