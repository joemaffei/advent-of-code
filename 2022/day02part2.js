/**
 * Code generated with ChatGPT. The only thing that needed my intervention was
 * the math for each roundScore. ChatGPT had some difficulty with the first term
 * of each addition.
 */

// Fetch the input data from the URL
fetch('https://adventofcode.com/2022/day/2/input')
  .then(response => response.text())
  // Split the input data at every newline character
  .then(data => data.split('\n'))
  // Ignore the last entry in the array
  .then(rounds => rounds.slice(0, -1))
  .then(rounds => {
    // Keep a running total of the scores
    let totalScore = 0;

    // Loop through each round
    for (const round of rounds) {
      // Split the round at the space character
      const [opponentShape, outcome] = round.split(' ');

      // Determine the score for the opponent's shape and the outcome of the round
      let roundScore = 0;
      switch (opponentShape) {
        case 'A':
          // If the opponent plays rock
          switch (outcome) {
            case 'X':
              // If we lose
              roundScore = 3 + 0;
              break;
            case 'Y':
              // If it's a draw
              roundScore = 1 + 3;
              break;
            case 'Z':
              // If we win
              roundScore = 2 + 6;
              break;
          }
          break;
        case 'B':
          // If the opponent plays paper
          switch (outcome) {
            case 'X':
              // If we lose
              roundScore = 1 + 0;
              break;
            case 'Y':
              // If it's a draw
              roundScore = 2 + 3;
              break;
            case 'Z':
              // If we win
              roundScore = 3 + 6;
              break;
          }
          break;
        case 'C':
          // If the opponent plays scissors
          switch (outcome) {
            case 'X':
              // If we lose
              roundScore = 2 + 0;
              break;
            case 'Y':
              // If it's a draw
              roundScore = 3 + 3;
              break;
            case 'Z':
              // If we win
              roundScore = 1 + 6;
              break;
          }
          break;
      }

      // Add the round score to the running total
      totalScore += roundScore;
    }

    // Print the final score
  console.log(totalScore);
});
