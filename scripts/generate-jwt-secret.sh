#!/bin/bash

# Generate a secure JWT secret for production use
# This script generates a cryptographically secure random string
# suitable for use as a JWT signing secret

set -e

echo "Generating secure JWT secret..."
echo ""

# Generate 64 bytes (512 bits) of random data and encode as base64
# This provides a very strong secret key
JWT_SECRET=$(openssl rand -base64 64 | tr -d '\n')

echo "✓ Generated JWT Secret (keep this secure!):"
echo ""
echo "$JWT_SECRET"
echo ""
echo "Add this to your .env.production file:"
echo "JWT_SECRET=$JWT_SECRET"
echo ""
echo "⚠️  IMPORTANT: Keep this secret secure and never commit it to version control!"
