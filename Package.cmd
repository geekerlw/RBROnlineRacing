@echo off
setlocal

rem Usage: Package.cmd 5.1.0.3
rem        This cmd creates a new Release\APPNAME_5.1.0.3.zip archive file for a release purposes
rem
rem Usage: Package.cmd 5.1.0.3 7z
rem        This cmd creates a new Release\APPNAME_5.1.0.3.7z archive file for a release purposes
rem

SET APPNAME=RBNHelper
SET DLLNAME=RBNHelper
SET VERSIONTAG=%~1

rem Archive file format zip or 7z
SET ARCHIVEFORMAT=%~2
if "%~2" == "" SET ARCHIVEFORMAT=zip
rem if "%~2" == "" SET ARCHIVEFORMAT=7z

SET RELEASE_FOLDER=Release\VER_%VERSIONTAG%
SET RELEASE_PKG=%APPNAME%_%VERSIONTAG%.%ARCHIVEFORMAT%

rem Download www.7Zip.org tool (or use just a 7za.exe cmdline tool without the 7z GUI suppport. Except that 7za uses a bit differernt cmd line option syntax)
SET ZIP_TOOL=C:\Program Files\7-Zip\7z.exe
if NOT EXIST "%ZIP_TOOL%" SET ZIP_TOOL=C:\Program Files (x86)\7-Zip\7z.exe
if NOT EXIST "%ZIP_TOOL%" SET ZIP_TOOL=7z.exe

echo [%DATE% %TIME%] %~nx0 %APPNAME% %VERSIONTAG%
echo [%DATE% %TIME%] Release folder  %RELEASE_FOLDER%
echo [%DATE% %TIME%] Release package %RELEASE_PKG%


if "%~1" == "" (
 echo Missing cmdline argument: versionTag
 echo Example: Package.cmd 1.14
 goto END
)

echo.
echo ZIP_TOOL=%ZIP_TOOL%
echo.
echo Do you want to create the pkg RELEASE\%RELEASE_PKG%?
echo Press CTRL-C to quit, other key to continue...
pause

rem Create release folders and copy the release versions of plugin files there
mkdir "%RELEASE_FOLDER%\"
mkdir "%RELEASE_FOLDER%\Plugins\"
mkdir "%RELEASE_FOLDER%\Plugins\%APPNAME%\"

xcopy /s "resource\*.*"      "%RELEASE_FOLDER%"
copy "README.md"             "%RELEASE_FOLDER%\Plugins\%APPNAME%\Readme.%APPNAME%.txt"
copy "LICENSE"               "%RELEASE_FOLDER%\Plugins\%APPNAME%\"
copy "target\i686-pc-windows-msvc\release\rbnhelper.dll"    "%RELEASE_FOLDER%\Plugins"


if NOT EXIST "%RELEASE_FOLDER%\Plugins\%APPNAME%\" (
   echo ERROR. %RELEASE_FOLDER% folder missing. Check release files and free disk space
   goto END
)

rem Compress RELEASE_FOLDER package
PUSHD "%RELEASE_FOLDER%\"
del "..\%RELEASE_PKG%"
"%ZIP_TOOL%" a -r -t%ARCHIVEFORMAT% -mx9 "..\%RELEASE_PKG%" *
POPD

rem Remove the temporary Release\VER_1.2.3.4\ version specific working folder
if NOT "%RELEASE_FOLDER%" == "" rmdir /S /Q "%RELEASE_FOLDER%\"

echo.
echo [%DATE% %TIME%] Release\%RELEASE_PKG% package created.
echo [%DATE% %TIME%] Verify the %ARCHIVEFORMAT% content before publishing it

:END

endlocal
