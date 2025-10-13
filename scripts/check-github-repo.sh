#!/bin/bash

# GitHub Repository Information Checker
# This script checks if GitHub Discussions is enabled and provides repository info

REPO_OWNER="AbletonPilot"
REPO_NAME="blog"

echo "================================"
echo "GitHub Repository Checker"
echo "Repository: $REPO_OWNER/$REPO_NAME"
echo "================================"
echo ""

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed."
    echo ""
    echo "Please install it from: https://cli.github.com/"
    echo ""
    echo "Or use Homebrew: brew install gh"
    echo "Or use apt: sudo apt install gh"
    echo ""
    echo "After installation, run: gh auth login"
    exit 1
fi

# Check if authenticated
if ! gh auth status &> /dev/null; then
    echo "❌ Not authenticated with GitHub CLI"
    echo ""
    echo "Please run: gh auth login"
    exit 1
fi

echo "✅ GitHub CLI is installed and authenticated"
echo ""

# Get repository information
echo "Fetching repository information..."
echo ""

REPO_INFO=$(gh api repos/$REPO_OWNER/$REPO_NAME)

# Check if repo exists and is accessible
if [ $? -ne 0 ]; then
    echo "❌ Cannot access repository. Please check:"
    echo "   - Repository name is correct"
    echo "   - You have access to the repository"
    echo "   - You are authenticated with the correct account"
    exit 1
fi

# Extract repository ID
REPO_ID=$(echo $REPO_INFO | jq -r '.node_id')
IS_PRIVATE=$(echo $REPO_INFO | jq -r '.private')
HAS_DISCUSSIONS=$(echo $REPO_INFO | jq -r '.has_discussions')

echo "Repository Information:"
echo "----------------------"
echo "Repository ID: $REPO_ID"
echo "Private: $IS_PRIVATE"
echo "Discussions Enabled: $HAS_DISCUSSIONS"
echo ""

# Check if repository is public
if [ "$IS_PRIVATE" = "true" ]; then
    echo "⚠️  WARNING: Repository is PRIVATE"
    echo "   Giscus requires a PUBLIC repository to work"
    echo "   Please make the repository public in Settings"
    echo ""
fi

# Check if discussions are enabled
if [ "$HAS_DISCUSSIONS" = "false" ]; then
    echo "⚠️  WARNING: GitHub Discussions is NOT enabled"
    echo ""
    echo "To enable Discussions:"
    echo "1. Go to: https://github.com/$REPO_OWNER/$REPO_NAME/settings"
    echo "2. Scroll to 'Features' section"
    echo "3. Check the 'Discussions' checkbox"
    echo ""
else
    echo "✅ GitHub Discussions is enabled!"
    echo ""
    
    # Try to get discussion categories
    echo "Fetching discussion categories..."
    echo ""
    
    # Note: This requires GraphQL API
    CATEGORIES=$(gh api graphql -f query='
      query($owner: String!, $name: String!) {
        repository(owner: $owner, name: $name) {
          discussionCategories(first: 10) {
            nodes {
              id
              name
              description
              isAnswerable
            }
          }
        }
      }
    ' -f owner=$REPO_OWNER -f name=$REPO_NAME)
    
    if [ $? -eq 0 ]; then
        echo "Discussion Categories:"
        echo "---------------------"
        echo $CATEGORIES | jq -r '.data.repository.discussionCategories.nodes[] | "Name: \(.name)\nID: \(.id)\nDescription: \(.description // "N/A")\n"'
        
        # Check for Announcements category
        ANNOUNCEMENTS_ID=$(echo $CATEGORIES | jq -r '.data.repository.discussionCategories.nodes[] | select(.name == "Announcements") | .id')
        
        if [ -n "$ANNOUNCEMENTS_ID" ] && [ "$ANNOUNCEMENTS_ID" != "null" ]; then
            echo "✅ Announcements category found!"
            echo ""
            echo "================================"
            echo "GISCUS CONFIGURATION VALUES"
            echo "================================"
            echo ""
            echo "Update src/components/giscus.rs with these values:"
            echo ""
            echo "data-repo-id: $REPO_ID"
            echo "data-category-id: $ANNOUNCEMENTS_ID"
            echo ""
        else
            echo "⚠️  Announcements category not found"
            echo ""
            echo "Please create an 'Announcements' category:"
            echo "1. Go to: https://github.com/$REPO_OWNER/$REPO_NAME/discussions"
            echo "2. Click on 'Categories' or the gear icon"
            echo "3. Create a new category named 'Announcements'"
            echo ""
        fi
    else
        echo "⚠️  Could not fetch discussion categories"
        echo "   This might be a permissions issue"
    fi
fi

echo ""
echo "================================"
echo "Next Steps:"
echo "================================"
echo ""
echo "1. Visit https://giscus.app to generate configuration"
echo "2. Use the IDs shown above to update src/components/giscus.rs"
echo "3. Test locally with: cargo leptos watch"
echo ""
echo "For detailed instructions, see: docs/giscus-setup-guide.md"
echo ""
