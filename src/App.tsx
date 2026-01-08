import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import "./App.css";
import {
  ObfuscateOptions,
  ProcessRequest,
  ProcessResult,
  defaultOptions,
} from "./types";

function App() {
  const [inputFile, setInputFile] = useState<string | null>(null);
  const [options, setOptions] = useState<ObfuscateOptions>(defaultOptions);
  const [count, setCount] = useState(1);
  const [outputDir, setOutputDir] = useState<string | null>(null);
  const [processing, setProcessing] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);
  const [isDragging, setIsDragging] = useState(false);

  const addLog = (msg: string) => {
    const time = new Date().toLocaleTimeString();
    setLogs((prev) => [...prev, `[${time}] ${msg}`]);
  };

  // 监听 Tauri 拖拽事件
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
            addLog(`已选择文件: ${file}`);
          } else {
            addLog(`❌ 请选择 ZIP 文件`);
          }
        }
      } else if (event.payload.type === "cancel") {
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
      addLog(`已选择文件: ${selected}`);
    }
  };

  const selectOutputDir = async () => {
    const selected = await open({ directory: true });
    if (selected) {
      setOutputDir(selected as string);
      addLog(`输出目录: ${selected}`);
    }
  };

  const handleProcess = async () => {
    if (!inputFile) {
      addLog("❌ 请先选择 Gerber 文件");
      return;
    }

    setProcessing(true);
    addLog("开始处理...");

    try {
      const request: ProcessRequest = {
        input_path: inputFile,
        output_dir: outputDir,
        count,
        options,
      };

      const result = await invoke<ProcessResult>("process_gerber", { request });

      if (result.success) {
        addLog(`✅ ${result.message}`);
        result.output_files.forEach((f) => addLog(`   📄 ${f}`));
      } else {
        addLog(`❌ 处理失败: ${result.message}`);
      }
    } catch (e) {
      addLog(`❌ 错误: ${e}`);
    } finally {
      setProcessing(false);
    }
  };

  const toggleOption = (key: keyof ObfuscateOptions) => {
    setOptions((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  const getFileName = (path: string) => path.split(/[/\\]/).pop() || path;

  return (
    <div className="app">
      <header className="header">
        <div className="logo">
          <h1>GhostPCB</h1>
        </div>
        <p className="subtitle">Gerber 混淆工具</p>
      </header>

      <main className="main">
        <section className="card">
          <h2 className="card-title">
            <span className="icon">📁</span>
            选择文件
          </h2>
          <div
            className={`drop-zone ${inputFile ? "has-file" : ""} ${
              isDragging ? "dragging" : ""
            }`}
            onClick={selectFile}
          >
            {inputFile ? (
              <div className="file-info">
                <span className="file-icon">📦</span>
                <span className="file-name">{getFileName(inputFile)}</span>
                <span className="file-change">点击更换</span>
              </div>
            ) : (
              <div className="drop-hint">
                <span className="drop-icon">⬆</span>
                <span>点击或拖放以选择 Gerber 文件</span>
              </div>
            )}
          </div>
        </section>

        <section className="card">
          <h2 className="card-title">
            <span className="icon">⚙️</span>
            混淆策略
          </h2>
          <div className="options-grid">
            {[
              {
                key: "timestamp" as const,
                label: "时间戳修改",
                desc: "替换文件内时间信息",
                risk: "safe",
              },
              {
                key: "silkscreen" as const,
                label: "丝印层扰动",
                desc: "微调丝印坐标",
                risk: "safe",
              },
              {
                key: "geometry" as const,
                label: "几何结构扰动",
                desc: "钻孔坐标偏移",
                risk: "low",
              },
              {
                key: "structure" as const,
                label: "文件结构混淆",
                desc: "插入冗余指令",
                risk: "safe",
              },
              {
                key: "physical" as const,
                label: "物理参数微调",
                desc: "外框尺寸调整",
                risk: "low",
              },
            ].map((opt) => (
              <label
                key={opt.key}
                className={`option-item ${options[opt.key] ? "active" : ""}`}
              >
                <input
                  type="checkbox"
                  checked={options[opt.key]}
                  onChange={() => toggleOption(opt.key)}
                />
                <div className="option-content">
                  <div className="option-header">
                    <span className="option-label">{opt.label}</span>
                    <span className={`risk-badge ${opt.risk}`}>
                      {opt.risk === "safe" ? "无风险" : "低风险"}
                    </span>
                  </div>
                  <span className="option-desc">{opt.desc}</span>
                </div>
                <div className="checkbox-visual">
                  <svg viewBox="0 0 24 24">
                    <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z" />
                  </svg>
                </div>
              </label>
            ))}
          </div>
        </section>

        <section className="card">
          <h2 className="card-title">
            <span className="icon">📊</span>
            生成设置
          </h2>
          <div className="settings-row">
            <div className="setting-item">
              <label>生成数量</label>
              <div className="number-input">
                <button onClick={() => setCount(Math.max(1, count - 1))}>
                  −
                </button>
                <input
                  type="number"
                  value={count}
                  onChange={(e) =>
                    setCount(Math.max(1, parseInt(e.target.value) || 1))
                  }
                  min={1}
                  max={99}
                />
                <button onClick={() => setCount(Math.min(99, count + 1))}>
                  +
                </button>
              </div>
            </div>
            <div className="setting-item output-setting">
              <label>输出目录</label>
              <div className="output-row">
                <span className="output-path">
                  {outputDir ? getFileName(outputDir) : "默认（原文件同级）"}
                </span>
                <button className="btn-secondary" onClick={selectOutputDir}>
                  选择...
                </button>
                {outputDir && (
                  <button
                    className="btn-clear"
                    onClick={() => setOutputDir(null)}
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

        {logs.length > 0 && (
          <section className="card log-card">
            <div className="log-header">
              <h2 className="card-title">
                <span className="icon">📋</span>
                处理日志
              </h2>
              <button className="btn-clear-log" onClick={() => setLogs([])}>
                清空
              </button>
            </div>
            <div className="log-content">
              {logs.map((log, i) => (
                <div key={i} className="log-line">
                  {log}
                </div>
              ))}
            </div>
          </section>
        )}
      </main>

      <footer className="footer">
        <span>GhostPCB v0.1.0</span>
      </footer>
    </div>
  );
}

export default App;
