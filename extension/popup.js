import init, { html_to_markdown } from "./pkg/html2md_ext.js";

const status = document.getElementById("status");
const output = document.getElementById("output");

async function readActiveTabHtml() {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  const [injection] = await chrome.scripting.executeScript({
    target: { tabId: tab.id },
    func: () => document.documentElement.outerHTML,
  });
  return injection.result;
}

try {
  await init();
  const html = await readActiveTabHtml();
  const markdown = html_to_markdown(html);
  output.value = markdown;
  await navigator.clipboard.writeText(markdown);
  status.textContent = "Markdown をコピーしました";
} catch (error) {
  status.textContent = `変換できませんでした: ${error.message ?? error}`;
}
