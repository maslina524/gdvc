$install_folder = "C:\Program Files\gdvc"
$install_path = "$install_folder\gdvc.exe"

Write-Host "Download to '$install_folder'" -ForegroundColor Cyan

if (-not (Test-Path $install_folder)) {
    New-Item -Path $install_folder -ItemType Directory -Force | Out-Null
}

if (-not (Test-Path $install_path)) {
    try {
        Invoke-WebRequest -Uri "https://github.com/maslina524/gdvc/releases/download/1.0.1/gdvc.exe" -OutFile $install_path
    }
    catch {
        Write-Host "Download error: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
    Write-Host "The Gdvc has been successfully installed!" -ForegroundColor Green
} else {
    Write-Host "Gdvc is already installed" -ForegroundColor Yellow
}
Write-Host ""

Write-Host "Installing Gdvc to the PATH" -ForegroundColor Cyan
$path = [Environment]::GetEnvironmentVariable("Path", "User")
$paths = $path -split ';' | Where-Object { $_ -ne '' }
if ($paths -contains $install_folder) {
    Write-Host "Gdvc is already installed in the PATH" -ForegroundColor Yellow
} else {
    [Environment]::SetEnvironmentVariable(
        "Path",
        [Environment]::GetEnvironmentVariable("Path", "User") + ";$install_folder",
        "User"
    )
    Write-Host "Gdvc has been successfully installed in the PATH!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Restart the terminal and you will be able to use Gdvc" -ForegroundColor Yellow
}
