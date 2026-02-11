# git-main.ps1
# Run this inside your project folder

# 1. Check if inside a Git repo
if (-not (Test-Path ".git")) {
    Write-Host "Not a git repository. Run inside a repo folder."
    exit
}

# 2. Ensure branch is main
git checkout main 2>$null

# 3. Ensure remote 'origin' exists
$remoteExists = git remote | Select-String "origin"
if (-not $remoteExists) {
    $repoUrl = Read-Host "Enter remote repo URL (e.g. https://github.com/Meu57/Titan_Engine_Physics.git)"
    git remote add origin $repoUrl
    Write-Host "Remote 'origin' added."
}

# 4. Ask for commit message
$commitMessage = Read-Host "Enter commit message"

# 5. Show status before staging
git status

# 6. Stage all changes
git add .

# 7. Commit with message
git commit -m $commitMessage

# 8. Fetch remote updates
git fetch origin main

# 9. Compare local vs remote commit hashes
$localHash = git rev-parse main
$remoteHash = git rev-parse origin/main

if ($localHash -ne $remoteHash) {
    Write-Host "Local branch is behind remote. Rebasing..."
    git pull --rebase origin main
} else {
    Write-Host "Local branch is up to date. No rebase needed."
}

# 10. Push to remote main
git push origin main
Write-Host "Changes pushed successfully to origin/main."
