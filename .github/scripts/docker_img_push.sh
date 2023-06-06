#!/bin/sh

# https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#labelling-container-images

IMAGE_NAME="net-hub"
IMAGE="net-hub"
OWNER="net-stalker"
REPO="net-monitor"
TAG="0.0.2"
USERNAME="github/packages"
PASSWORD=""

# Function to display help information
display_help() {
  echo "Usage: docker_img_push [options]"
  echo "Options:"
  echo "  -h, --help     Display this help message"
  echo "  -im, --image_name the desired name for your Docker image"
  echo "  -i, --image the desired image name"
  echo "  -o, --owner GitHub username or organization"
  echo "  -r, --repo the name of the repository"
  echo "  -t, --tag the desired tag (e.g., version number)"
  echo "  -u, --username GitHub username"
  echo "  -p, --password your Personal Access Token (PAT)"
  echo ""
  echo "Example:"
  echo "  docker_img_push net-hub net-hub net-stalker net-monitor 0.0.2 github/packages ghp_OsD55aVNKsivc40xSHB7q7canRI4ZC1bXdlO"
}

# Function to display error message
display_error() {
  echo "Error: $1"
  display_help
  exit 1
}

# Check if the script was invoked with the help option
if [ "$1" = "-h" ] || [ "$1" = "--help" ]; then
  display_help
  exit 0
fi

# Check the number of input parameters
if [ $# -eq 0 ]; then
  display_error "No arguments provided"
fi

# Process the input parameters
while [ $# -gt 0 ]; do
  case "$1" in
  -im | --image_name)
    IMAGE_NAME="$2"
    shift 2
    ;;
  -i | --image)
    IMAGE="$2"
    shift 2
    ;;
  -o | --owner)
    OWNER="$2"
    shift 2
    ;;
  -r | --repo)
    REPO="$2"
    shift 2
    ;;
  -t | --tag)
    TAG="$2"
    shift 2
    ;;
  -u | --username)
    USERNAME="$2"
    shift 2
    ;;
  -p | --password)
    PASSWORD="$2"
    shift 2
    ;;
  *)
    display_error "Invalid option: $1"
    ;;
  esac
done

# Build the Docker image
echo "Build the Docker image with params IMAGE=${IMAGE} IMAGE_NAME=${IMAGE_NAME}"
docker build --build-arg PROJ_NAME=${IMAGE} -t ${IMAGE_NAME} .

# Tag the Docker image
echo "Tag the Docker image with params IMAGE_NAME=${IMAGE_NAME} OWNER=${OWNER} REPO=${REPO} IMAGE=${IMAGE} TAG=${TAG}"
docker tag ${IMAGE_NAME} ghcr.io/${OWNER}/${REPO}/${IMAGE}:${TAG}

# Authenticate with GitHub Container Registry:
# Before pushing the Docker image to GitHub Container Registry, you need to authenticate with a personal access token (PAT) that has the appropriate permissions.
# Generate a PAT from your GitHub account settings with the read:packages and write:packages scopes.

# Log in to GitHub Container Registry: Run the following command to authenticate and log in to GitHub Container Registry:
# Replace "USERNAME" with your GitHub username and "PERSONAL_ACCESS_TOKEN" with your PAT.
echo "Log in to GitHub Container Registry"
docker login ghcr.io -u${USERNAME} -p${PASSWORD}

# Push the Docker image
echo "Push the Docker image with params OWNER=${OWNER} REPO=${REPO} IMAGE=${IMAGE} TAG=${TAG}"
docker push ghcr.io/${OWNER}/${REPO}/${IMAGE}:${TAG}