import { useEffect } from 'react';
import config from './api/config';
import './App.css';
import LeftBar from './components/app/LeftBar';
import { MessageList } from './components/app/MessageList';
import CommandPlate from './components/CommandPlate';

function App() {
    useEffect(() => {
    }, [])

    return (
        <div className="Appx min-h-screen max-h-screen text-sm">
            {/* command plate */}
            <CommandPlate />
            {/* Layout start  */}
            <div className='app-container grid grid-cols-6 grid-rows-4 min-h-screen '>
                <div className='col-span-1 row-span-4 '>
                    <LeftBar />
                </div>
                <div className='col-span-5 row-span-3 border-b-2 overflow-y-scroll'>
                    <MessageList />
                </div>
                <div className='col-span-5 row-span-1'></div>
            </div>
        </div>
    );
}

export default App;
