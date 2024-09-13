$NuePath = "$env:LOCALAPPDATA\.nue\node"

$CurrentPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User) -split ';'

if ($CurrentPath -contains $NuePath) {
  Write-Error "Nue is already in `$Path."
  return
}

$NewPath = @($NuePath) + $CurrentPath
[System.Environment]::SetEnvironmentVariable("Path", $NewPath -join ";", [System.EnvironmentVariableTarget]::User)

Write-Output "Nue is now added to user path. Restart your shell to start using Node."
