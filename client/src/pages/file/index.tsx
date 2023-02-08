import React, { useEffect, useState } from "react"
import { create_download_link, delete_file, FileStat, read_dir } from "../../apis/file";
import path from 'path-browserify';
import style from './index.module.less';
import Preview from "./components/preview";
import { Popover } from "../../components/popover";
import { useRefresh } from "../../hooks/common";
import LoadingBar from "./components/loading-bar";

export default function FilePage() {
  let [files, setFiles] = useState<any[]>([]);
  let [currentDir, setCurrentDir] = useState('');
  const [signal, reloadFiles] = useRefresh();
  let [previewing, setPreviewing] = useState<FileStat>();
  let [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    (async () => {
      setIsLoading(true);
      let data = await read_dir(currentDir);
      setIsLoading(false);
      setFiles(data);
    })();
  }, [currentDir, signal]);

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
          <LoadingBar loading={isLoading} />
          <Breadcumb onJumpPath={(p) => setCurrentDir(p)} currentPath={currentPath} />
          {
            files.length ?
              <FileList onReload={reloadFiles} files={files} currentDir={currentDir} onClickFile={onClickFile} />
              : <EmptyList />
          }
        </div>
        : <div>
          <div className={style['preview-title-bar']}><span>{previewing.name}</span><span style={{ cursor: 'pointer' }} onClick={() => setPreviewing(undefined)}>X</span></div>
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
          <span className={style['breadcumb-item-sep']}>/</span>
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

function FileList({ files, onClickFile, currentDir, onReload }: { files: FileStat[], onClickFile: (file: FileStat) => void, currentDir: string, onReload: () => void }) {

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
            <DeleteBtn dir={currentDir} file={file} onDeleteFinish={onReload} />
            {
              file.is_file &&
              <a
                className={style['action-btn']}
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


function DeleteBtn({ dir, file, onDeleteFinish }: { dir: string, file: FileStat, onDeleteFinish: () => void }) {

  const [showDeleteComfirm, setShowDeleteComfirm] = useState(false);

  useEffect(() => {
    const onClick = () => {
      setShowDeleteComfirm(false);
    };
    window.addEventListener('click', onClick, false);
    return () => {
      window.removeEventListener('click', onClick, false);
    };
  }, []);

  const comfirmContent = <div onClick={e => e.stopPropagation()} className={style['comfirm-content']}>
    Comfirm to delete<button onClick={async () => {
      await delete_file(dir, file.name);
      onDeleteFinish();
      setShowDeleteComfirm(false);
    }}>OK</button>
  </div>;

  return <Popover content={comfirmContent} show={showDeleteComfirm}>
    <span
      className={style['action-btn']}
      onClick={(e) => {
        e.stopPropagation();
        setShowDeleteComfirm(true);
      }}
    >
      删除
    </span>
  </Popover>
}

function EmptyList() {
  return <div className={style['empty-list']}>No file in this directory</div>
}
