import React, { useEffect, useState } from "react"
import { create_download_link, FileStat, read_dir } from "../../apis/file";
import path from 'path-browserify';
import style from './index.module.less';
import Preview from "./components/preview";

export default function FilePage() {
  let [files, setFiles] = useState<any[]>([]);
  let [currentDir, setCurrentDir] = useState('');
  let [previewing, setPreviewing] = useState<FileStat>();

  useEffect(() => {
    (async () => {
      let data = await read_dir(currentDir);
      setFiles(data);
    })();
  }, [currentDir]);

  const onClickFile = (file: FileStat) => {
    if (file.is_dir) {
      setCurrentDir(path.join(currentDir, file.name));
    } else {
      setPreviewing(file);
    }
  }

  const currentPath = previewing ? path.join(currentDir, previewing.name) : currentDir;

  return <div className={style['file-page']}>
    {
      !previewing ?
        <div>
          <Breadcumb onJumpPath={(p) => setCurrentDir(p)} currentPath={currentPath} />
          {
            files.length ?
              <FileList files={files} currentDir={currentDir} onClickFile={onClickFile} />
              : <EmptyList />
          }
        </div>
        : <div>
          <div className={style['preview-title-bar']}><span>{previewing.name}</span><span onClick={() => setPreviewing(undefined)}>X</span></div>
          <Preview file={previewing} dir={currentDir} />
        </div>
    }
  </div >
}

function Breadcumb({ onJumpPath, currentPath }: { onJumpPath: (p: string) => void, currentPath: string }) {
  let acc = '';
  return <div className={style.breadcumb}>
    <span className={style['breadcumb-item']} onClick={() => onJumpPath('')}>Home</span>
    {
      currentPath && currentPath.split('/').map((p, i) => {
        acc = acc + (i === 0 ? '' : '/') + p;
        const cur_path = acc;
        return <React.Fragment key={cur_path}>
          <span>/</span>
          <span className={style['breadcumb-item']} key={cur_path} onClick={() => {
            onJumpPath(cur_path);
          }}>
            {p}
          </span>
        </React.Fragment>;
      })
    }
  </div>
}

function FileList({ files, onClickFile, currentDir }: { files: FileStat[], onClickFile: (file: FileStat) => void, currentDir: string }) {

  return <div>
    {
      files.map(file => {
        return <div
          className={style['file-item']}
          key={file.name}
          data-filename={file.name}
          data-isdir={file.is_dir}
        >
          <span
            onClick={() => onClickFile(file)}
            className={style['left-area']}>
            {file.name}
          </span>
          <span className={style['right-area']}>
            {
              file.is_file &&
              <a
                download={file.name}
                target="_blank"
                rel="noreferrer"
                href={create_download_link(currentDir, file.name)}>
                下载
              </a>
            }
          </span>
        </div>
      })
    }
  </div>
}


function EmptyList() {
  return <div className={style['empty-list']}>No file in this directory</div>
}
