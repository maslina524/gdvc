$install_folder = "C:\Program Files\gdvc"
$exe_path = "$install_folder\gdvc.exe"
$doc_path = "$install_folder\doc.zip"
$repo = "maslina524/gdvc"

Write-Host "Installing to '$install_folder'" -ForegroundColor Cyan

if (-not (Test-Path $install_folder)) {
    New-Item -Path $install_folder -ItemType Directory -Force | Out-Null
}

try {
    Write-Host "Fetching latest release information..." -ForegroundColor Cyan
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$repo/releases/latest" -Method Get -ErrorAction Stop
} catch {
    Write-Host "Failed to fetch release data: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

$exe_asset = $release.assets | Where-Object { $_.name -eq "gdvc.exe" }
$doc_asset = $release.assets | Where-Object { $_.name -eq "doc.zip" }

if (-not $exe_asset) {
    Write-Host "ERROR: 'gdvc.exe' not found in the latest release. Installation aborted." -ForegroundColor Red
    exit 1
}

if (-not $doc_asset) {
    Write-Host "WARNING: 'doc.zip' not found in the latest release. Documentation will not be installed." -ForegroundColor Yellow
}

try {
    Write-Host "Downloading gdvc.exe..." -ForegroundColor Cyan
    Invoke-WebRequest -Uri $exe_asset.browser_download_url -OutFile $exe_path -ErrorAction Stop
    Write-Host "gdvc.exe downloaded successfully." -ForegroundColor Green
} catch {
    Write-Host "ERROR: Failed to download gdvc.exe: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

if ($doc_asset) {
    try {
        Write-Host "Downloading doc.zip..." -ForegroundColor Cyan
        Invoke-WebRequest -Uri $doc_asset.browser_download_url -OutFile $doc_path -ErrorAction Stop
        Write-Host "doc.zip downloaded successfully." -ForegroundColor Green

        if (Test-Path "$install_folder\doc") {
            Remove-Item -Path "$install_folder\doc" -Force -Recurse
        }
        Expand-Archive -Path "$doc_path" -DestinationPath "$install_folder\doc"
        Remove-Item -Path "$doc_path" -Force
    } catch {
        Write-Host "WARNING: Failed to download doc.zip: $($_.Exception.Message)" -ForegroundColor Yellow
    }
}

Write-Host "`nAdding Gdvc to the PATH..." -ForegroundColor Cyan
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathEntries = $userPath -split ';' | Where-Object { $_ -ne '' }

if ($pathEntries -contains $install_folder) {
    Write-Host "Gdvc is already in the PATH." -ForegroundColor Yellow
} else {
    try {
        [Environment]::SetEnvironmentVariable(
            "Path",
            $userPath + ";$install_folder",
            "User"
        )
        Write-Host "Successfully added Gdvc to the PATH!" -ForegroundColor Green
        Write-Host "Please restart your terminal to use Gdvc." -ForegroundColor Yellow
    } catch {
        Write-Host "Failed to update PATH: $($_.Exception.Message)" -ForegroundColor Red
    }
}

Write-Host "`nInstallation completed." -ForegroundColor Green