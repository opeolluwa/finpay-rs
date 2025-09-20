- Find an API for realtime recording  (WS, WebRTC, MediaStream API)-> [ validation :: mic works or nor no audio device ]
- break recording into chunks and process in prarallel (web / service workers)
- even driven approach / event streaming for Rust/JS 
- result and state management


### EVent streaming  
- Media streaming API -> HTTP Server (WS/WebRTC) [:: not optimized] (embed server + HTTP streaming)
- Event API (https://v2.tauri.app/develop/calling-frontend/)


### State management 
1. Use JS Media stream API  to 
    - check if recorder exist
    - record 
2. Web/service workers to break the media into chunks 
3. Use event to process the stuff and return response. 
4. 