import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { openUrl } from "@tauri-apps/plugin-opener";
import { check } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import "./App.css";
import {
  ObfuscateOptions,
  ProcessRequest,
  ProcessResult,
  defaultOptions,
} from "./types";

const APP_VERSION = "v1.2.0";
const GITHUB_URL = "https://github.com/Nitmi/GhostPCB";

function App() {
  const [inputFile, setInputFile] = useState<string | null>(null);
  const [options, setOptions] = useState<ObfuscateOptions>(defaultOptions);
  const [count, setCount] = useState(1);
  const [countInput, setCountInput] = useState("1");
  const [outputDir, setOutputDir] = useState<string | null>(null);
  const [processing, setProcessing] = useState(false);
  const [status, setStatus] = useState<{
    type: "idle" | "success" | "error";
    message: string;
  }>({
    type: "idle",
    message: "",
  });
  const [isDragging, setIsDragging] = useState(false);
  const [showAbout, setShowAbout] = useState(false);
  const [updateInfo, setUpdateInfo] = useState<{
    available: boolean;
    version?: string;
    downloading?: boolean;
    progress?: number;
  }>({ available: false });

  // 检查更新
  useEffect(() => {
    const checkUpdate = async () => {
      try {
        const update = await check();
        if (update) {
          setUpdateInfo({ available: true, version: update.version });
        }
      } catch (e) {
        console.error("检查更新失败:", e);
      }
    };
    checkUpdate();
  }, []);

  // 执行更新
  const handleUpdate = async () => {
    try {
      const update = await check();
      if (!update) return;

      setUpdateInfo((prev) => ({ ...prev, downloading: true, progress: 0 }));

      let downloaded = 0;
      let contentLength = 0;

      await update.downloadAndInstall((event) => {
        if (event.event === "Started") {
          contentLength =
            (event.data as { contentLength?: number }).contentLength || 0;
        } else if (event.event === "Progress") {
          downloaded += (event.data as { chunkLength: number }).chunkLength;
          if (contentLength > 0) {
            const progress = Math.round((downloaded / contentLength) * 100);
            setUpdateInfo((prev) => ({ ...prev, progress }));
          }
        }
      });

      await relaunch();
    } catch (e) {
      console.error("更新失败:", e);
      setUpdateInfo((prev) => ({ ...prev, downloading: false }));
      setStatus({ type: "error", message: "更新失败: " + String(e) });
    }
  };

  useEffect(() => {
    const webview = getCurrentWebviewWindow();
    const unlisten = webview.onDragDropEvent((event) => {
      if (event.payload.type === "over") {
        setIsDragging(true);
      } else if (event.payload.type === "drop") {
        setIsDragging(false);
        const paths = event.payload.paths;
        if (paths && paths.length > 0) {
          const file = paths[0];
          if (file.toLowerCase().endsWith(".zip")) {
            setInputFile(file);
            setStatus({ type: "idle", message: "" });
          } else {
            setStatus({ type: "error", message: "请选择 ZIP 文件" });
          }
        }
      } else if (event.payload.type === "leave") {
        setIsDragging(false);
      }
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const selectFile = async () => {
    const selected = await open({
      filters: [{ name: "Gerber ZIP", extensions: ["zip"] }],
      multiple: false,
    });
    if (selected) {
      setInputFile(selected as string);
      setStatus({ type: "idle", message: "" });
    }
  };

  const selectOutputDir = async () => {
    const selected = await open({ directory: true });
    if (selected) {
      setOutputDir(selected as string);
    }
  };

  const handleProcess = async () => {
    if (!inputFile) {
      setStatus({ type: "error", message: "请先选择 Gerber 文件" });
      return;
    }
    setProcessing(true);
    setStatus({ type: "idle", message: "" });
    try {
      const request: ProcessRequest = {
        input_path: inputFile,
        output_dir: outputDir,
        count,
        options,
      };
      const result = await invoke<ProcessResult>("process_gerber", { request });
      if (result.success) {
        setStatus({
          type: "success",
          message: `成功生成 ${result.output_files.length} 个文件`,
        });
      } else {
        setStatus({ type: "error", message: result.message });
      }
    } catch (e) {
      setStatus({ type: "error", message: String(e) });
    } finally {
      setProcessing(false);
    }
  };

  const toggleOption = (key: keyof ObfuscateOptions) => {
    setOptions((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  const getFileName = (path: string) => path.split(/[/\\]/).pop() || path;

  const enabledCount = Object.values(options).filter(Boolean).length;

  return (
    <div className="app">
      <header className="header">
        <div className="logo">
          <h1>GhostPCB</h1>
          <span className="divider">|</span>
          <span className="subtitle">Gerber 混淆工具</span>
        </div>
        <div className="header-actions">
          {updateInfo.available && (
            <button
              className={`btn-update ${
                updateInfo.downloading ? "downloading" : ""
              }`}
              onClick={handleUpdate}
              disabled={updateInfo.downloading}
              title={`新版本 ${updateInfo.version} 可用`}
            >
              {updateInfo.downloading ? (
                <>
                  <span className="spinner-small"></span>
                  {updateInfo.progress}%
                </>
              ) : (
                <>
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width="16"
                    height="16"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    strokeWidth="2"
                    strokeLinecap="round"
                    strokeLinejoin="round"
                  >
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
                    <polyline points="7 10 12 15 17 10" />
                    <line x1="12" x2="12" y1="15" y2="3" />
                  </svg>
                  更新
                </>
              )}
            </button>
          )}
          <button
            className="btn-about"
            onClick={() => setShowAbout(true)}
            title="关于"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            >
              <circle cx="12" cy="12" r="10" />
              <line x1="12" x2="12" y1="8" y2="12" />
              <line x1="12" x2="12.01" y1="16" y2="16" />
            </svg>
          </button>
        </div>
      </header>

      <main className="main">
        <div className="left-panel">
          <section className="card file-card">
            <h2 className="card-title">Gerber 文件</h2>
            <div
              className={`drop-zone ${inputFile ? "has-file" : ""} ${
                isDragging ? "dragging" : ""
              }`}
              onClick={selectFile}
            >
              {inputFile ? (
                <div className="file-info">
                  <span className="file-icon">📦</span>
                  <div className="file-details">
                    <span className="file-name">{getFileName(inputFile)}</span>
                    <span className="file-path">{inputFile}</span>
                  </div>
                </div>
              ) : (
                <div className="drop-hint">
                  <span className="drop-icon">📁</span>
                  <span>点击选择或拖拽 ZIP 文件</span>
                </div>
              )}
            </div>
          </section>

          <section className="card settings-card">
            <h2 className="card-title">生成设置</h2>
            <div className="settings-list">
              <div className="setting-item">
                <label>生成数量</label>
                <div className="number-input">
                  <button
                    onClick={() => {
                      const newCount = Math.max(1, count - 1);
                      setCount(newCount);
                      setCountInput(String(newCount));
                    }}
                  >
                    −
                  </button>
                  <input
                    type="text"
                    value={countInput}
                    onChange={(e) => {
                      const val = e.target.value;
                      if (val === "" || /^\d+$/.test(val)) {
                        setCountInput(val);
                        if (val !== "") {
                          const num = parseInt(val);
                          if (num >= 1 && num <= 99) {
                            setCount(num);
                          }
                        }
                      }
                    }}
                    onBlur={() => {
                      if (countInput === "" || parseInt(countInput) < 1) {
                        setCount(1);
                        setCountInput("1");
                      } else if (parseInt(countInput) > 99) {
                        setCount(99);
                        setCountInput("99");
                      }
                    }}
                  />
                  <button
                    onClick={() => {
                      const newCount = Math.min(99, count + 1);
                      setCount(newCount);
                      setCountInput(String(newCount));
                    }}
                  >
                    +
                  </button>
                </div>
              </div>
              <div className="setting-item">
                <label>输出目录</label>
                <div className="output-row">
                  <span className="output-path" title={outputDir || undefined}>
                    {outputDir ? getFileName(outputDir) : "原文件同级目录"}
                  </span>
                  <button
                    className="btn-icon"
                    onClick={selectOutputDir}
                    title="选择目录"
                  >
                    📂
                  </button>
                  {outputDir && (
                    <button
                      className="btn-icon btn-clear"
                      onClick={() => setOutputDir(null)}
                      title="重置"
                    >
                      ✕
                    </button>
                  )}
                </div>
              </div>
            </div>
          </section>

          <button
            className={`btn-process ${processing ? "processing" : ""}`}
            onClick={handleProcess}
            disabled={processing || !inputFile}
          >
            {processing ? (
              <>
                <span className="spinner"></span>
                处理中...
              </>
            ) : (
              <>开始处理</>
            )}
          </button>

          {status.message && (
            <div className={`status-bar ${status.type}`}>
              <span className="status-icon">
                {status.type === "success" ? "✓" : "!"}
              </span>
              <span>{status.message}</span>
            </div>
          )}
        </div>

        <div className="right-panel">
          <section className="card options-card">
            <div className="options-header">
              <h2 className="card-title">混淆策略</h2>
              <span className="options-count">{enabledCount}/5 已启用</span>
            </div>
            <div className="options-list">
              {[
                {
                  key: "timestamp" as const,
                  label: "时间戳修改",
                  desc: "替换文件内的日期时间信息",
                  risk: "safe",
                },
                {
                  key: "silkscreen" as const,
                  label: "丝印层扰动",
                  desc: "微调丝印层坐标 (±0.05mm)",
                  risk: "safe",
                },
                {
                  key: "structure" as const,
                  label: "文件结构混淆",
                  desc: "插入冗余指令和随机注释",
                  risk: "safe",
                },
                {
                  key: "geometry" as const,
                  label: "几何结构扰动",
                  desc: "钻孔坐标随机偏移 (±0.02mm)",
                  risk: "low",
                },
                {
                  key: "physical" as const,
                  label: "物理参数微调",
                  desc: "外框尺寸微调 (±0.01mm)",
                  risk: "low",
                },
              ].map((opt) => (
                <label
                  key={opt.key}
                  className={`option-item ${options[opt.key] ? "active" : ""}`}
                >
                  <div className="option-content">
                    <div className="option-header">
                      <span className="option-label">{opt.label}</span>
                      <span className={`risk-badge ${opt.risk}`}>
                        {opt.risk === "safe" ? "无风险" : "低风险"}
                      </span>
                    </div>
                    <span className="option-desc">{opt.desc}</span>
                  </div>
                  <div className="toggle-switch">
                    <input
                      type="checkbox"
                      checked={options[opt.key]}
                      onChange={() => toggleOption(opt.key)}
                    />
                    <span className="toggle-slider"></span>
                  </div>
                </label>
              ))}
            </div>
          </section>
        </div>
      </main>

      {showAbout && (
        <div className="about-overlay" onClick={() => setShowAbout(false)}>
          <div className="about-modal" onClick={(e) => e.stopPropagation()}>
            <div className="about-header">
              <span className="about-title">关于</span>
              <button className="btn-close" onClick={() => setShowAbout(false)}>
                <svg viewBox="0 0 24 24" width="20" height="20">
                  <path
                    fill="currentColor"
                    d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"
                  />
                </svg>
              </button>
            </div>
            <div className="about-content">
              <div className="about-icon">
                <img src="/icon.png" alt="GhostPCB" width="64" height="64" />
              </div>
              <h2 className="about-app-name">GhostPCB</h2>
              <div className="about-desc-group">
                <p>一个 Gerber 混淆工具</p>
                <p>异化 Gerber 文件</p>
                <p>但生产出来是同样的 PCB</p>
              </div>
              <p className="about-version">{APP_VERSION}</p>
              <button
                className="btn-github"
                onClick={() => openUrl(GITHUB_URL)}
              >
                <svg viewBox="0 0 24 24" width="18" height="18">
                  <path
                    fill="currentColor"
                    d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                  />
                </svg>
                GitHub
              </button>
            </div>
            <div className="about-footer">
              <p>
                声明：此软件仅供个人学习使用，不可用于商业用途，任何侵犯嘉立创等厂商商业利益的行为，本软件概不负责！
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
