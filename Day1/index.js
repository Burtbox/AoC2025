import { dir } from "node:console";
import fs from "node:fs"

fs.readFile('./Day1/input.txt', (err, data) => {
    if(err) throw err;
    const array = data.toString().split("\n");
    let currentPosition = 50;
    let answer = 0;
    for(let i in array) {
        console.log(array[i]);

        let direction = array[i].charAt(0);
        console.log("direction", direction);

        let change = array[i].replace("L", "").replace("R", "");

        console.log("change", change);

        if(direction == "L"){
            currentPosition = currentPosition - Number.parseInt(change)
        } else{ 
            currentPosition = currentPosition + Number.parseInt(change)
        }

        if(currentPosition < 0 ) {
            currentPosition = 100000 + currentPosition
        }

        console.log("premodulo", currentPosition)
        currentPosition = currentPosition % 100

        console.log("post modulo", currentPosition)

        if(currentPosition === 0)
        {
            answer++
        }

        console.log("answer", answer)
    }
});
