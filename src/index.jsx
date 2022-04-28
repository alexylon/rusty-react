import React, {useState} from "react";
import ReactDOM from "react-dom";
import {Box, TextField, Button, Stack} from '@mui/material';

const wasm = import("../build/rusty_react");

wasm.then(m => {
    const App = () => {
        const [name, setName] = useState("");
        const [a, setA] = useState(0);
        const [b, setB] = useState(0);
        const [sum, setSum] = useState(0);
        const [stateSum, setStateSum] = useState(0);
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
            let numbers = JSON.stringify({num1: a, num2: b});
            console.log(numbers);
            setSum(m.sum_two_ints(numbers));
        }
        const getSum = () => {
            setStateSum(m.get_sum());
        }

        return (
            <Box
                component="form"
                sx={{
                    '& > :not(style)': {m: 1, width: '250ch'},
                }}
                noValidate
                autoComplete="off"
                marginLeft={5}
                marginTop={6}
            >
                <Box marginTop={6}>
                    <Box>
                        <TextField id="name" label="Name" variant="standard" onChange={(e) => handleChange(e)}/>
                    </Box>
                    <Box marginTop={2} marginBottom={6}>
                        <Button variant="contained" onClick={() => handleClick()}>Say hello!</Button>
                    </Box>
                </Box>
                <Stack spacing={2} direction="row">
                    <TextField id="number1" label="Number 1" variant="standard"
                               inputProps={{inputMode: 'numeric', pattern: '[0-9]*'}}
                               onChange={(e) => handleChangeA(e)}/>
                    <TextField id="number2" label="Number 2" variant="standard"
                               inputProps={{inputMode: 'numeric', pattern: '[0-9]*'}}
                               onChange={(e) => handleChangeB(e)}/>
                    <Box>
                        <h3>Sum from Rust: {sum}</h3>
                    </Box>
                </Stack>
                <Box marginTop={2}>
                    <Button variant="contained" onClick={() => handleSum()}>Sum numbers</Button>
                </Box>
                <Stack spacing={2} direction="row">
                    <Box marginTop={2}>
                        <Button variant="contained" onClick={() => getSum()}>Get sum</Button>
                    </Box>
                    <Box>
                        <h3>Sum from Rust state: {stateSum}</h3>
                    </Box>
                </Stack>
            </Box>
        );
    };

    ReactDOM.render(<App/>, document.getElementById("root"));
});
