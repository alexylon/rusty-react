import React, {useState} from "react";
import ReactDOM from "react-dom";

const wasm = import("../build/rusty_react");

wasm.then(m => {
    const App = () => {
        const [name, setName] = useState("");
        const [a, setA] = useState(0);
        const [b, setB] = useState(0);
        const [sum, setSum] = useState(0);
        const handleChange = (e) => {
            setName(e.target.value);
        }
        const handleClick = () => {
            m.welcome(name);
        }

        const handleChangeA = (e) => {
            setA(e.target.value);
        }
        const handleChangeB = (e) => {
            setB(e.target.value);
        }
        const handleSum = () => {
            setSum(m.sum_two_ints(a, b));
        }

        return (
            <>
                <div>
                    <h1>Hello there</h1>
                    <button onClick={m.big_computation}>Run Computation</button>
                </div>
                <br/>
                <div>
                    <input type="text" onChange={handleChange}/>
                    <button onClick={handleClick}>Say hello!</button>
                </div>
                <br/>
                <div>
                    <input type="number" onChange={handleChangeA}/>
                    <input type="number" onChange={handleChangeB}/>
                    <button onClick={handleSum}>Sum!</button>
                    <br/>
                    <br/>
                    <h3>Sum from Rust: {sum}</h3>
                </div>
            </>
        );
    };

    ReactDOM.render(<App/>, document.getElementById("root"));
});
