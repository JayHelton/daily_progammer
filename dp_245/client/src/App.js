import React, {useEffect, useState} from 'react';
import logo from './logo.svg';
import './App.css';



function App() {
    const [wasm, setWasm] = useState();
    const [translation, setTranslation] = useState();

    useEffect(() => {
        try {
            import("dp_245").then(wasm => {
                debugger
                setWasm(wasm);
            })
        } catch (err) {
            console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
        }
    }, [])

    return (
        <div className="App">
            <input onChange={e => setTranslation(wasm.translate(e.target.value))}/>
            <div>
                <h2>{translation}</h2>
            </div>
        </div>
    );
}

export default App;
