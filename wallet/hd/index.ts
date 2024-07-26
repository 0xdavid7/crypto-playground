import {
  deriveEd25519Path,
  keyPairFromSeed,
  mnemonicNew,
  mnemonicToHDSeed,
  mnemonicValidate,
} from "ton-crypto";
import { MemoryStorage, StorageKey } from "./storage";

const password = "123abc";

class DerivationPath {
  static default = "m/44'/607'/0'";
  /**
   *
   * @description Because the `deriveEd25519Path` function of the `ton-crypto` package is used `hardened childs`, so just derive the path to number array.
   */
  static toNumberArray(): number[] {
    return [44, 607, 0];
  }
}

const store = new MemoryStorage();

const randomInt = (min: number, max: number) => {
  return Math.floor(Math.random() * (max - min + 1) + min);
};

const eightIndexes = () => {
  let indexes: number[] = [];
  for (let i = 0; i < 8; i++) {
    let index = randomInt(0, 23);
    while (indexes.includes(index)) {
      index = randomInt(0, 23);
    }
    indexes.push(index);
  }
  return indexes;
};

const generateMnemonic = async () => {
  try {
    let mnemonics: string[] = await mnemonicNew(24, password);
    let valid = await mnemonicValidate(mnemonics, password);
    if (!valid) {
      throw new Error("Invalid Mnemonic");
    }

    store.set({
      [StorageKey.MNEMONIC]: mnemonics,
    });

    console.log("mnemonics:", mnemonics);
  } catch (error) {
    console.log(error);
  }
};

const verifyMnemonic = async (mode: string = "skip") => {
  const mnemonics = (await store.get(StorageKey.MNEMONIC)) as any;
  if (!mnemonics) {
    return false;
  }

  const valid = await mnemonicValidate(mnemonics, password);
  if (!valid) {
    return false;
  }

  if (mode === "skip") {
    return true;
  }

  //   let indexes = eightIndexes();
  let indexes = [0, 1, 2, 3, 4, 5, 6, 7];
  let lines = 8;
  const prompt = `Let's verify your mnemonic.\nEnter the word at index #${
    indexes[8 - lines] + 1
  }: `;
  process.stdout.write(prompt);
  lines--;

  for await (const line of process.stdin) {
    if (lines === 0) {
      break;
    }
    console.log(`You typed: ${line}`);
    if (`${line}`.trim() !== mnemonics[indexes[8 - lines - 1]]) {
      return false;
    }
    process.stdout.write(
      `Enter the word at index #${indexes[8 - lines] + 1}: `
    );
    lines--;
  }

  return true;
};

const run = async () => {
  await generateMnemonic();
  const verified = await verifyMnemonic();
  if (verified) {
    console.log("Mnemonic is valid");
  } else {
    console.log("Mnemonic is not valid");
  }

  // recover steps
  const mnemonics = (await store.get(StorageKey.MNEMONIC)) as any;

  const mnemonicHDSeed = await mnemonicToHDSeed(mnemonics, password);

  const derivedSeed = await deriveEd25519Path(
    mnemonicHDSeed,
    DerivationPath.toNumberArray()
  );

  console.log("mnemonicHDSeed:", mnemonicHDSeed.toString("hex"));

  console.log("derivedSeed:", derivedSeed.toString("hex"));

  const { publicKey, secretKey } = await keyPairFromSeed(derivedSeed);

  console.log("publicKey:", publicKey.toString("hex"));
  console.log("secretKey:", secretKey.toString("hex"));
};

run();