import React, { useCallback, useEffect, useMemo, useState } from "react"
import { create_download_link, delete_file, FileStat, read_dir } from "../../apis/file";
import path from 'path-browserify';
import style from './index.module.less';
import Preview from "./components/preview";
import { Popover } from "../../components/popover";
import { useRefresh } from "../../hooks/common";
import LoadingBar from "./components/loading-bar";
import moment from 'moment';
import { formatFileSize } from "../../utils/formatter";
import classnames from 'classnames';
import { useLocation, useNavigate } from 'react-router-dom';

export default function FilePage() {
  let [files, setFiles] = useState<any[]>([]);
  const [signal, reloadFiles] = useRefresh();
  let [isLoading, setIsLoading] = useState(false);

  const location = useLocation();
  const currentDir = location.state?.currentDir || '';
  const previewing = location.state?.previewing;
  const history = useNavigate();

  const gotoDir = (dir: string = currentDir) => {
    history('/', { state: { currentDir: dir } });
  };

  const reload = async (dir: string = currentDir) => {
    setIsLoading(true);
    let data = await read_dir(dir);
    setIsLoading(false);
    setFiles(data);
  };

  const setPreviewing = (file: FileStat) => {
    history('/', { state: { previewing: file } })
  }

  useEffect(() => {
    gotoDir('');
  }, []);

  useEffect(() => {
    reload();
  }, [signal]);

  useEffect(() => {
    (async () => {
      const state = location.state;
      if (state && state.currentDir !== undefined) {
        await reload(state.currentDir);
      }
    })();
  }, [location]);

  const onClickFile = (file: FileStat) => {
    if (file.is_dir) {
      gotoDir(path.join(currentDir, file.name));
    } else {
      setPreviewing(file);
    }
  };

  const currentPath = previewing ? path.join(currentDir, previewing.name) : currentDir;

  return <div className={style['file-page']}>
    {
      !previewing ?
        <div>
          <LoadingBar loading={isLoading} />
          <Breadcumb onJumpPath={(p) => gotoDir(p)} currentPath={currentPath} />
          {
            files.length ?
              <FileList onReload={reloadFiles} files={files} currentDir={currentDir} onClickFile={onClickFile} />
              : <EmptyList />
          }
        </div>
        : <div className={style['preview']}>
          <div className={style['preview-title-bar']}><span>{previewing.name}</span><span style={{ cursor: 'pointer' }} onClick={() => gotoDir()}>X</span></div>
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
          <span title={p} className={style['breadcumb-item']} key={cur_path} onClick={() => {
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

  const actionsMenu = (file: FileStat) => <div className={style['action-menu']}>
    <div className={style['action-btn']}>
      <DeleteBtn dir={currentDir} file={file} onDeleteFinish={onReload} />
    </div>
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
  </div>;

  let copiedFiles = useMemo(() => files.slice(), [files]);

  let [sortKey, setSortKey] = useState<keyof FileStat | undefined>();
  let [sortType, setSortType] = useState<-1 | 1>(1);

  if (sortKey) {
    copiedFiles.sort((a, b) => {
      return a[sortKey!] < b[sortKey!] ? -1 * sortType : 1 * sortType;
    });
  }

  const [animationClass, setAnimationClass] = useState(true);

  const setSort = (key?: keyof FileStat) => {
    if (sortKey === key) {
      if (sortType === 1) {
        setSortType(-1);
      } else {
        setSortKey(undefined);
      }
    } else {
      setSortType(1);
      setSortKey(key);
    }
  }

  useEffect(() => {
    setAnimationClass(false);
    requestAnimationFrame(() => {
      setAnimationClass(true);
    });
  }, [files]);

  return <div className={classnames(style['file-list'], style['fade-in-start'], { [style['ease-in']]: animationClass })}>
    <div className={style['file-head']}>
      <div onClick={() => setSort('name')}>
        文件名
        {sortKey === 'name' && <span className={classnames(style['sort-icon'], { [style['revert-icon']]: sortType === -1 })}>&gt;</span>}
      </div>
      <div onClick={() => setSort('created')}>
        创建时间
        {sortKey === 'created' && <span className={classnames(style['sort-icon'], { [style['revert-icon']]: sortType === -1 })}>&gt;</span>}
      </div>
      <div onClick={() => setSort('size')}>
        大小
        {sortKey === 'size' && <span className={classnames(style['sort-icon'], { [style['revert-icon']]: sortType === -1 })}>&gt;</span>}
      </div>
    </div>
    <div>
      {
        copiedFiles.map(file => {
          return <div
            className={style['file-item']}
            key={file.name}
            data-filename={file.name}
            data-isdir={file.is_dir}
          >
            <div
              className={style['left-area']}>
              <span
                onClick={() => onClickFile(file)}
              >
                {file.name}
              </span>
              <Popover auto={true} content={actionsMenu(file)}>
                <span>...</span>
              </Popover>
            </div>
            <div className={style['right-area']}>
              {moment.unix(file.created / 1000 >> 0).format('YYYY/MM/DD')}
            </div>
            <div>
              {file.is_file ? formatFileSize(file.size) : '-'}
            </div>
          </div>
        })
      }
    </div>
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
