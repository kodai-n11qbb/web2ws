#!/bin/bash
# Quick demo and verification script for web2ws

set -e

echo "🚀 Web2WS Quick Start Demo"
echo "=========================="
echo ""

# Check if server is already running
if lsof -i :9001 > /dev/null 2>&1; then
    echo "⚠️  Port 9001 already in use. Killing existing process..."
    pkill -f "target/release/web2ws" || true
    sleep 1
fi

# Build
echo "🔨 Building web2ws..."
cargo build --release --quiet
echo "✅ Build complete"
echo ""

# Start server
echo "🌍 Starting server on http://localhost:9001..."
./target/release/web2ws --bind 127.0.0.1:9001 --fps 60 &
SERVER_PID=$!
sleep 2

echo "✅ Server started (PID: $SERVER_PID)"
echo ""

# Test HTTP endpoints
echo "📋 Testing HTTP endpoints..."
echo ""

echo "1️⃣  Testing /static/sender.html..."
if curl -s http://localhost:9001/static/sender.html | grep -q "📷 Camera Sender"; then
    echo "   ✅ Sender page loads correctly"
else
    echo "   ❌ Sender page failed"
fi

echo ""
echo "2️⃣  Testing /static/viewer.html..."
if curl -s http://localhost:9001/static/viewer.html | grep -q "📺 Video Viewer"; then
    echo "   ✅ Viewer page loads correctly"
else
    echo "   ❌ Viewer page failed"
fi

echo ""
echo "3️⃣  Testing / redirect..."
if curl -s http://localhost:9001/ | grep -q "📷 Camera Sender"; then
    echo "   ✅ Root redirects to sender"
else
    echo "   ❌ Root redirect failed"
fi

echo ""
echo "📊 Server Status"
echo "================"
echo ""
echo "Server is running on: http://localhost:9001"
echo ""
echo "Quick Links:"
echo "  🔗 Sender: http://localhost:9001/static/sender.html"
echo "  📺 Viewer: http://localhost:9001/static/viewer.html"
echo ""

echo "⏱️  Running for 10 seconds to check FPS..."
echo ""

# Capture server output for 10 seconds to verify FPS counter
timeout 10 tail -f /dev/null &
WAIT_PID=$!
sleep 10
wait $WAIT_PID 2>/dev/null || true

echo ""
echo "✅ Demo complete!"
echo ""
echo "To stop server: kill $SERVER_PID"
echo "Or press Ctrl+C"

# Keep server running until interrupted
wait $SERVER_PID
