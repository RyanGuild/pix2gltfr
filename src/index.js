import React from "react"
import { render } from 'react-dom';
import { App } from './app';
console.log(document.getElementById('root'));
render(<App/>, document.getElementById('root'), () => console.log("react-dom render complete"));