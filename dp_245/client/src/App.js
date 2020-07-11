import React, {useEffect, useState} from 'react';
import logo from './logo.svg';
import './App.css';


function App() {
    const [wasm, setWasm] = useState();
    const [decoder, setDecoder] = useState();
    const [phrase, setPhrase] = useState();
    const [translation, setTranslation] = useState();

    useEffect(() => {
        try {
            import("dp_245").then(wasm => {
                setWasm(wasm);
            })
        } catch (err) {
            console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
        }
    }, [])

    return (
        <div className="App">
            <p>Decoder Map</p>
            <input onChange={e => setDecoder(e.target.value)}/>
            <p>Phrase</p>
            <input onChange={e => setPhrase(e.target.value)}/>
            <button onClick={() => setTranslation(wasm.translate(decoder, phrase))}>Translate</button>
            <div>
                <h2>{translation}</h2>
            </div>
        </div>
    );
}

export default App;
