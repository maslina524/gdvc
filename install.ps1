$InstallFolder = "C:\Program Files\gdvc"
$ExePath = "$InstallFolder\gdvc.exe"
$DocPath = "$InstallFolder\doc"
$SourcePath = "$InstallFolder\source"
$Repo = "maslina524/gdvc"
$MaxLength = 30

function Dot-Print {
    param(
        [string]$Message
    )

    Write-Host "$($Message) " -NoNewLine
    for ($i = 0; $i -le $MaxLength - $Message.Length; $i++) {
        Write-Host "." -NoNewLine
    }
    Write-Host " " -NoNewLine
}

function Download-Source {
    param(
        [string]$Repository,
        [string]$Folder,
        [string]$Destination
    )

    Write-Host "Installing the doc directory from GitHub:"
    Invoke-WebRequest -Uri "https://api.github.com/repos/$($Repository)/zipball/main" -OutFile "$($SourcePath).zip"

    Dot-Print "  Extracting archive"
    Expand-Archive -Path "$($SourcePath).zip" -DestinationPath "$($SourcePath)" -Force
    Write-Host "Successfully"

    $SubfolderName = Get-ChildItem -Path "$($SourcePath)" -Directory | Select-Object -ExpandProperty Name

    Dot-Print "  Copying directory"
    Remove-Item -Path $DocPath -Recurse -Force
    Copy-Item -Path "$($SourcePath)\$($SubfolderName)\$($Folder)" -Destination $DocPath -Recurse -Force
    
    Remove-Item -Path "$($SourcePath).zip" -Recurse -Force
    Remove-Item -Path "$($SourcePath)" -Recurse -Force

    Write-Host "Successfully`n"
}

function Download-Exe {
    param(
        [string]$Repository,
        [string]$Folder,
        [string]$Destination
    )

    Write-Host "Installing gdvc.exe from the latest release:"
    $Response = Invoke-RestMethod "https://api.github.com/repos/$($Repository)/releases/latest"
    $DownloadUrl = ($Response.assets | Where-Object { $_.name -eq "gdvc.exe" }).browser_download_url

    Dot-Print "  Downloading"
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $Destination

    Write-Host "Successfully`n"
}

function AddTo_Path {
    param(
        [string]$Directory
    )

    Dot-Print "Adding directory to PATH"
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $PathEntries = $UserPath -split ';' | Where-Object { $_ -ne '' }

    if ($PathEntries -contains $Directory) {
        Write-Host "Already"
    } else {
        try {
            [Environment]::SetEnvironmentVariable(
                "Path",
                $userPath + ";$install_folder",
                "User"
            )
            Write-Host "Successfully"
            Write-Host "Please restart your terminal to use Gdvc."
        } catch {
            Write-Host "Failed to update PATH: $($_.Exception.Message)" -ForegroundColor Red
        }
    }
}

Download-Exe -Repository $Repo -Destination $ExePath
Download-Source -Repository $Repo -Folder "doc" -Destination $DocPath
AddTo_Path -Directory $InstallFolder