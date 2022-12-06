const fs = require('fs');
const readline = require('readline');

async function processLineByLine() {
  const fileStream = fs.createReadStream('input');

  const rl = readline.createInterface({
    input: fileStream,
    crlfDelay: Infinity
  });

  let pairs = 0;

  for await (const line of rl) {
    let s = line.split(",");

    [left1, right1] = s[0].split("-");
    [left2, right2] = s[1].split("-");

    if ((+left1 <= +left2 && +right1 >= +right2) || (+left2 <= +left1 && +right2 >= +right1)) {
        pairs++;
    }
  }

  console.log(pairs);
}

processLineByLine();
