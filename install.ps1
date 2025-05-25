# install.ps1

# Target install directory
$installDir = "$env:LOCALAPPDATA\tell"

# Create directory if missing
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

# Copy tell.exe and tell.bat to installDir
Copy-Item -Path "tell.exe" -Destination $installDir -Force
Copy-Item -Path "tell.bat" -Destination $installDir -Force

# Read current user PATH
$oldPath = [Environment]::GetEnvironmentVariable("Path", "User")

# Add installDir if missing
if (-not ($oldPath.Split(';') -contains $installDir)) {
    $newPath = $oldPath + ";" + $installDir
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
    Write-Host "Added $installDir to your user PATH."
} else {
    Write-Host "$installDir already in your user PATH."
}

Write-Host "Installation complete."
Write-Host "Please restart your terminal or log out/in to apply PATH changes."
