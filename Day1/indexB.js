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

        while(change > 0){
            if(direction == "L") {
                currentPosition--;
            } else {
                currentPosition ++
            }

            if(Math.abs(currentPosition) == 100) 
            {
                currentPosition = 0
            }
            if(currentPosition === 0){
                answer++
            }
            change-- 
        }

        console.log("answer", answer)
    }
});
