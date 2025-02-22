import { invoke } from "@tauri-apps/api/core";

type KanaItem = {
  image: string;
  romaji: string;
};

let kanaList: KanaItem[] = [];
let shuffledKanaList: KanaItem[] = [];
let currentKanaIndex = 0;

const kanaImage = document.getElementById("kana-image") as HTMLImageElement;
const userInput = document.getElementById("user-input") as HTMLInputElement;
const feedback = document.getElementById("feedback") as HTMLParagraphElement;
const form = document.getElementById("answer-form") as HTMLFormElement;

// 从后端获取假名列表
async function fetchKanaList() {
  kanaList = await invoke<KanaItem[]>("get_kana_list");
  startNewRound();
}

// 开启新一轮并洗牌
function startNewRound() {
  shuffledKanaList = shuffleArray([...kanaList]); // 复制并洗牌
  currentKanaIndex = 0;
  loadNextKana();
}

// 洗牌算法 (Fisher-Yates)
function shuffleArray(array: KanaItem[]): KanaItem[] {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]]; // 交换
  }
  return array;
}

// 加载下一个假名
function loadNextKana() {
  feedback.textContent = "";
  userInput.value = "";

  if (currentKanaIndex >= shuffledKanaList.length) {
    // 完成一轮后，开始新一轮
    feedback.textContent = "🎉 Round Completed! Starting new round...";
    setTimeout(startNewRound, 1500); // 延迟开始新一轮
    return;
  }

  const currentKana = shuffledKanaList[currentKanaIndex];
  kanaImage.src = currentKana.image;
}

// 检查用户输入
form.addEventListener("submit", (e) => {
  e.preventDefault();

  const currentKana = shuffledKanaList[currentKanaIndex];
  const userAnswer = userInput.value.trim().toLowerCase();

  if (userAnswer === currentKana.romaji.toLowerCase()) {
    feedback.textContent = "✅ Correct!";
  } else {
    feedback.textContent = `❌ Incorrect! Correct answer: ${currentKana.romaji}`;
  }

  currentKanaIndex++;
  setTimeout(loadNextKana, 1000); // 1.5秒后加载下一个
});

// 初始化
window.addEventListener("DOMContentLoaded", fetchKanaList);
