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

Download-Source -Repo $Repo -Folder "doc" -Destination $DocPath