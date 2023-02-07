import React, { MouseEvent, useEffect, useState } from "react"
import { create_download_link, read_dir } from "../../apis/file";
import path from 'path-browserify';
import style from './index.module.less';

export default function FilePage() {
  let [files, setFiles] = useState<any[]>([]);
  let [currentDir, setCurrentDir] = useState('');

  useEffect(() => {
    (async () => {
      let data = await read_dir(currentDir);
      setFiles(data);
    })();
  }, [currentDir]);

  const onClickFile = (e: MouseEvent<HTMLDivElement>) => {
    let el = e.currentTarget as HTMLDivElement;
    let filename = el.dataset.filename;
    let isDir = el.dataset.isdir;
    if (isDir === 'true') {
      const new_file_path = path.join(currentDir, filename || '');
      setCurrentDir(new_file_path);
    }
  }

  let acc = '';
  return <div className={style['file-page']} >
    <div className={style.breadcumb}>
      <span className={style['breadcumb-item']} onClick={() => setCurrentDir('')}>Home</span>
      {
        currentDir && currentDir.split('/').map(p => {
          acc = acc + p;
          return <React.Fragment key={acc}>
            <span>/</span>
            <span className={style['breadcumb-item']} key={acc} onClick={() => {
              setCurrentDir(acc)
            }}>
              {p}
            </span>
          </React.Fragment>;
        })
      }
    </div>
    {
      files.length ? files.map(file => {
        return <div
          className={style['file-item']}
          onClick={onClickFile}
          key={file.name}
          data-filename={file.name}
          data-isdir={file.is_dir}
        >
          <span className={style['left-area']}>
            {file.name}
          </span>
          <span className={style['right-area']}>
            {
              file.is_file && <a download={file.name} target="_blank" rel="noreferrer" href={create_download_link(currentDir, file.name)}>下载</a>
            }
          </span>
        </div>
      })
      : <EmptyList />
    }
  </div>
}

function EmptyList() {
  return <div className={style['empty-list']}>No file in this directory</div>
}