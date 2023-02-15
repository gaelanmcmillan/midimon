import os
NEW_PLUGIN = "/Users/dogzone/dev/2023/audio/Midimon/target/bundled/midimon.vst3"
OLD_PLUGIN = "/Library/Audio/Plug-Ins/VST3/Dev/midimon.vst3"

print("Deleting old plugin")
os.system(f"rm -rf {OLD_PLUGIN}")
print("Copying new plugin")
os.system(f"cp -r {NEW_PLUGIN} {OLD_PLUGIN}")
print("Opening Reaper")
os.system("cd /Applications/REAPER64.app/Contents/MacOS && ./REAPER")