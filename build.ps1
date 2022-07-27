if ((Get-Command rustup -errorAction SilentlyContinue)){
    curl https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe `
    -o "rustup-init.exe"
    
    , "1" * 2 | ./rustup-init.exe

}


if (!(Test-Path -Path "$Env:USERPROFILE\.punch")){
        mkdir -p "$Env:USERPROFILE\.punch\trash"

        mkdir "$Env:USERPROFILE\.punch\bin"

        echo "~/.punch created in home" 
}
cargo build --release
curl https://aka.ms/vs/16/release/vs_BuildTools.exe `
    -o "rustup-init.exevs_BuildTools.exe"
    
.\vs_buildtool.exe --add Microsoft.VisualStudio.Workload.MSBuildTools^ 
                 --add Microsoft.VisualStudio.Workload.VCTools^ 
                 --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64^ 
                 --add Microsoft.VisualStudio.Component.Windows10SDK.18362^ 
                 --add Microsoft.VisualStudio.Component.VC.CMake.Project^ 
                 --add Microsoft.VisualStudio.Component.TestTools.BuildTools^ 
                 --add Microsoft.VisualStudio.Component.VC.ASAN^ 
                 --add Microsoft.VisualStudio.Component.VC.140 | Out-Null

mv .\target\release\punch.exe $Env:USERPROFILE\.punch\bin\


$Env:PATH = "$Env:PATH; $ENV:USERPROFILE\.punch\bin\"

$Env:PATH >> safe.txt

setx PATH "$Env:PATH; $ENV:USERPROFILE\.punch\bin\" -m

echo "build complete-- type: punch -h"