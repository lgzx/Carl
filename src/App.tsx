import { useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';
import config from './api/config';
import './App.css';
import LeftBar from './components/app/LeftBar';
import { MessageList } from './components/app/MessageList';
import CommandPlate from './components/CommandPlate';
import { Editor } from './components/Editor';

function App() {

    const [param, setParam] = useSearchParams();

    useEffect(() => {
        document.addEventListener("contextmenu", evt => evt.preventDefault())
    }, [])

    return (
        <div className="Appx min-h-screen max-h-screen text-sm">
            {/* command plate */}
            <CommandPlate />
            {/* Layout start  */}
            <div className='app-container grid grid-cols-8 grid-rows-4 min-h-screen '>
                <div className='col-span-2 row-span-4 '>
                    <LeftBar broker={param.get("broker") as string} />
                </div>
                <div className='col-span-6 row-span-3 border-b-2 overflow-y-auto'>
                    <MessageList />
                </div>
                <div className='col-span-6 row-span-1'>
                    <Editor />
                </div>
            </div>
        </div>
    );
}

export default App;
