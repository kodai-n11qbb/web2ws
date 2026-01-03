class WebSocketVideoReceiver {
    constructor() {
        this.ws = null;
        this.canvas = document.getElementById('videoCanvas');
        this.ctx = this.canvas.getContext('2d');
        this.status = document.getElementById('status');
        this.connectBtn = document.getElementById('connectBtn');
        this.disconnectBtn = document.getElementById('disconnectBtn');
        this.qualitySlider = document.getElementById('qualitySlider');
        this.qualityValue = document.getElementById('qualityValue');
        this.fpsSlider = document.getElementById('fpsSlider');
        this.fpsValue = document.getElementById('fpsValue');
        
        // Stats
        this.fpsCounter = document.getElementById('fpsCounter');
        this.frameCounter = document.getElementById('frameCounter');
        this.latencyCounter = document.getElementById('latencyCounter');
        this.dataRateCounter = document.getElementById('dataRateCounter');
        
        // Internal state
        this.isConnected = false;
        this.frameCount = 0;
        this.lastFrameTime = Date.now();
        this.fps = 0;
        this.latency = 0;
        this.dataRate = 0;
        this.lastDataTime = Date.now();
        this.dataBytes = 0;
        
        this.setupEventListeners();
        this.updateStatus('disconnected');
    }
    
    setupEventListeners() {
        this.connectBtn.addEventListener('click', () => this.connect());
        this.disconnectBtn.addEventListener('click', () => this.disconnect());
        
        this.qualitySlider.addEventListener('input', (e) => {
            this.qualityValue.textContent = e.target.value;
            this.sendQualitySettings();
        });
        
        this.fpsSlider.addEventListener('input', (e) => {
            this.fpsValue.textContent = e.target.value;
            this.sendQualitySettings();
        });
    }
    
    connect() {
        if (this.isConnected) return;
        
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/ws`;
        
        this.updateStatus('connecting');
        console.log('Connecting to:', wsUrl);
        
        try {
            this.ws = new WebSocket(wsUrl);
            
            this.ws.onopen = () => {
                console.log('WebSocket connected');
                this.isConnected = true;
                this.updateStatus('connected');
                this.connectBtn.disabled = true;
                this.disconnectBtn.disabled = false;
                this.sendQualitySettings();
            };
            
            this.ws.onmessage = (event) => {
                this.handleMessage(event.data);
            };
            
            this.ws.onclose = () => {
                console.log('WebSocket disconnected');
                this.isConnected = false;
                this.updateStatus('disconnected');
                this.connectBtn.disabled = false;
                this.disconnectBtn.disabled = true;
            };
            
            this.ws.onerror = (error) => {
                console.error('WebSocket error:', error);
                this.updateStatus('disconnected');
            };
            
        } catch (error) {
            console.error('Failed to connect:', error);
            this.updateStatus('disconnected');
        }
    }
    
    disconnect() {
        if (!this.isConnected) return;
        
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
        
        this.isConnected = false;
        this.updateStatus('disconnected');
        this.connectBtn.disabled = false;
        this.disconnectBtn.disabled = true;
    }
    
    handleMessage(data) {
        const startTime = Date.now();
        
        // Handle binary data (JPEG frames)
        if (data instanceof Blob) {
            const reader = new FileReader();
            reader.onload = (e) => {
                const imageData = e.target.result;
                this.displayFrame(imageData);
                this.updateStats(startTime, data.size);
            };
            reader.readAsArrayBuffer(data);
        } else if (typeof data === 'string') {
            // Handle text messages (control messages)
            try {
                const message = JSON.parse(data);
                this.handleControlMessage(message);
            } catch (e) {
                console.log('Received text message:', data);
            }
        }
    }
    
    displayFrame(imageData) {
        const img = new Image();
        img.onload = () => {
            // Set canvas size to match image
            if (this.canvas.width !== img.width || this.canvas.height !== img.height) {
                this.canvas.width = img.width;
                this.canvas.height = img.height;
            }
            
            // Draw image to canvas
            this.ctx.drawImage(img, 0, 0);
        };
        img.src = URL.createObjectURL(new Blob([imageData], { type: 'image/jpeg' }));
    }
    
    handleControlMessage(message) {
        console.log('Control message:', message);
        // Handle any control messages from server
    }
    
    sendQualitySettings() {
        if (!this.isConnected) return;
        
        const settings = {
            type: 'quality_settings',
            quality: parseInt(this.qualitySlider.value),
            targetFps: parseInt(this.fpsSlider.value)
        };
        
        this.ws.send(JSON.stringify(settings));
    }
    
    updateStats(startTime, dataSize) {
        const endTime = Date.now();
        this.latency = endTime - startTime;
        
        // Update FPS
        this.frameCount++;
        const now = Date.now();
        const timeDiff = now - this.lastFrameTime;
        
        if (timeDiff >= 1000) {
            this.fps = Math.round((this.frameCount * 1000) / timeDiff);
            this.frameCount = 0;
            this.lastFrameTime = now;
        }
        
        // Update data rate
        this.dataBytes += dataSize;
        const dataTimeDiff = now - this.lastDataTime;
        if (dataTimeDiff >= 1000) {
            this.dataRate = Math.round((this.dataBytes * 1000) / dataTimeDiff / 1024);
            this.dataBytes = 0;
            this.lastDataTime = now;
        }
        
        // Update UI
        this.fpsCounter.textContent = this.fps;
        this.frameCounter.textContent = this.frameCount;
        this.latencyCounter.textContent = this.latency;
        this.dataRateCounter.textContent = this.dataRate;
    }
    
    updateStatus(status) {
        this.status.className = `status ${status}`;
        
        switch (status) {
            case 'connected':
                this.status.textContent = 'Status: Connected';
                break;
            case 'connecting':
                this.status.textContent = 'Status: Connecting...';
                break;
            case 'disconnected':
                this.status.textContent = 'Status: Disconnected';
                break;
            default:
                this.status.textContent = 'Status: Unknown';
        }
    }
}

// Initialize the application
document.addEventListener('DOMContentLoaded', () => {
    const receiver = new WebSocketVideoReceiver();
    
    // Auto-connect when page loads
    setTimeout(() => {
        receiver.connect();
    }, 1000);
});
