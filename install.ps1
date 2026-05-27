$InstallFolder = "C:\Program Files\gdvc"
$ExePath = "$InstallFolder\gdvc.exe"
$DocPath = "$InstallFolder\doc"
$SourcePath = "$InstallFolder\source"
$Repo = "maslina524/gdvc"

function Download-Source {
    param(
        [string]$Repository,
        [string]$Folder,
        [string]$Destination
    )

    Write-Host "Installing the doc directory from GitHub..."
    Invoke-WebRequest -Uri "https://api.github.com/repos/$($Repository)/zipball/main" -OutFile "$($SourcePath).zip"

    Write-Host "  Extracting archive"
    Expand-Archive -Path "$($SourcePath).zip" -DestinationPath "$($SourcePath)" -Force

    $SubfolderName = Get-ChildItem -Path "$($SourcePath)" -Directory | Select-Object -ExpandProperty Name

    Write-Host "  Copying directory"
    Remove-Item -Path $DocPath -Recurse -Force
    Copy-Item -Path "$($SourcePath)\$($SubfolderName)\$($Folder)" -Destination $DocPath -Recurse -Force
    
    Remove-Item -Path "$($SourcePath).zip" -Recurse -Force
    Remove-Item -Path "$($SourcePath)" -Recurse -Force

    Write-Host "  ...Successfully"
}

function Download-Exe {
    param(
        [string]$Repository,
        [string]$Folder,
        [string]$Destination
    )

    Write-Host "Installing gdvc.exe from the latest release..."
    $Response = Invoke-RestMethod "https://api.github.com/repos/$($Repository)/releases/latest"
    $DownloadUrl = ($Response.assets | Where-Object { $_.name -eq "gdvc.exe" }).browser_download_url

    Write-Host "  Downloading"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $Destination

    Write-Host "  ...Successfully"
}

function AddTo_Path {
    param(
        [string]$Directory
    )

    Write-Host "Adding directory to PATH..."
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathEntries = $UserPath -split ';' | Where-Object { $_ -ne '' }

    if ($PathEntries -contains $Directory) {
        Write-Host "  Gdvc is already in the PATH"
    } else {
        try {
            [Environment]::SetEnvironmentVariable(
                "Path",
                $userPath + ";$install_folder",
                "User"
            )
            Write-Host "  ...Successfully"
            Write-Host "  Please restart your terminal to use Gdvc."
        } catch {
            Write-Host "  Failed to update PATH: $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

Download-Exe -Repository $Repo -Destination $ExePath
Download-Source -Repository $Repo -Folder "doc" -Destination $DocPath
AddTo_Path -Directory $InstallFolder