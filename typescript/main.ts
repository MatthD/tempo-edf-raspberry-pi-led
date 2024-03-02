import { BinaryValue, Gpio } from "onoff"; // Assuming you're using Node.js with npm install onoff
import { CronJob } from "cron";
import axios from "axios"; // Using Axios for HTTP requests

type ApiCodeTempoResponse = {
  dateJour: string;
  codeJour: TarifColor; // Using 'TarifColor' instead of number
  periode: string;
};

enum TarifColor {
  Unknown = 0,
  Blue = 1,
  White = 2,
  Red = 3,
}

// Fetch and set RGB color of the day
const job = new CronJob("5 6,22 * * *", main);

main().then(() => {
  job.start();
});

async function main() {
  const color = await fetchEDFColorOfTheDay();
  console.log("So today we are in %s color", color);
  if (color) {
    const red = color === TarifColor.Red || color === TarifColor.White ? 1 : 0; // Assuming colors are returned with properties 'red', 'green' and 'blue'
    const green = color === TarifColor.White ? 1 : 0;
    const blue = color === TarifColor.Blue ? 1 : 0;

    setRGBColor(red, green, blue);
  } else {
    console.error("No EDF color of the day available");
  }
}

// Function to set RGB color
function setRGBColor(red: BinaryValue, green: BinaryValue, blue: BinaryValue) {
  const redPin = new Gpio(process.env.RED_PIN_NB as unknown as number, "out");
  const greenPin = new Gpio(
    process.env.GREEN_PIN_NB as unknown as number,
    "out"
  );
  const bluePin = new Gpio(process.env.BLUE_PIN_NB as unknown as number, "out");
  redPin.writeSync(red);
  greenPin.writeSync(green);
  bluePin.writeSync(blue);
  process.on("SIGINT", (_) => {
    redPin.unexport();
    greenPin.unexport();
    bluePin.unexport();
  });
}

// Function to get EDF color of the day (You'll need to replace this with your actual API endpoint)
async function fetchEDFColorOfTheDay() {
  try {
    const response = await axios.get<ApiCodeTempoResponse>(
      "https://www.api-couleur-tempo.fr/api/jourTempo/today"
    ); // Replace 'https://your-edf-api/color' with the actual EDF color of the day API endpoint
    return response.data.codeJour;
  } catch (error) {
    console.log(`Error fetching EDF color: ${error}`);
    return null; // Return null if there was an error fetching the color
  }
}
