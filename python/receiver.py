#!/usr/bin/env python3
"""
WebSocket経由でwebcam映像を受信して表示するPythonクライアント
"""

import asyncio
import websockets
import cv2
import numpy as np
from io import BytesIO

async def receive_frames(uri: str = "ws://127.0.0.1:8080/ws"):
    """WebSocketからフレームを受信して表示"""
    try:
        async with websockets.connect(uri) as websocket:
            print(f"Connected to {uri}")
            
            while True:
                # フレームデータを受信
                frame_data = await websocket.recv()
                
                # JPEGデータをnumpy配列に変換
                nparr = np.frombuffer(frame_data, np.uint8)
                frame = cv2.imdecode(nparr, cv2.IMREAD_COLOR)
                
                if frame is not None:
                    # フレームを表示
                    cv2.imshow('Webcam Stream', frame)
                    
                    # 'q'キーで終了
                    if cv2.waitKey(1) & 0xFF == ord('q'):
                        break
                else:
                    print("Failed to decode frame")
                    
    except websockets.exceptions.ConnectionClosed:
        print("Connection closed")
    except Exception as e:
        print(f"Error: {e}")
    finally:
        cv2.destroyAllWindows()

def main():
    """メイン関数"""
    import sys
    
    uri = sys.argv[1] if len(sys.argv) > 1 else "ws://127.0.0.1:8080/ws"
    print(f"Connecting to {uri}...")
    
    asyncio.run(receive_frames(uri))

if __name__ == "__main__":
    main()

