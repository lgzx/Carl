import { emit, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import './App.css';

interface Msg {
    payload: Payload
}

interface Payload {
    content?: string
}

function App() {
    return (
        <div className="Appx">
        </div>
    );
}

export default App;
