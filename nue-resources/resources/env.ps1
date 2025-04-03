$NuePath = "$env:LocalAppData\Programs\nue\node"

$CurrentPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::User) -split ';'

if ($CurrentPath -contains $NuePath) {
  Write-Error "Nue is already in `$Path."
  return
}

$NewPath = @($NuePath) + $CurrentPath
[System.Environment]::SetEnvironmentVariable("Path", $NewPath -join ";", [System.EnvironmentVariableTarget]::User)

$HWND_BROADCAST = [IntPtr] 0xffff
$WM_SETTINGCHANGE = 0x1a
$SMTO_ABORTIFHUNG = 0x0002
if (-not ("Win32.NativeMethods" -as [Type])) {
  Add-Type -Namespace Win32 -Name NativeMethods -MemberDefinition @"
        [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
        public static extern IntPtr SendMessageTimeout(
          IntPtr hWnd,
          uint Msg,
          UIntPtr wParam,
          string lParam,
          uint fuFlags,
          uint uTimeout,
          out UIntPtr lpdwResult
        );
"@
}

if ([Win32.Nativemethods]::SendMessageTimeout($HWND_BROADCAST, $WM_SETTINGCHANGE, [UIntPtr]::Zero, "Environment", $SMTO_ABORTIFHUNG, 5000, [ref] [UIntPtr]::Zero) -eq 0) {
  Write-Warning "Failed to broadcast environment change. Please restart your shell to start using Node."
}

Write-Output "Nue is now added to the user path."
