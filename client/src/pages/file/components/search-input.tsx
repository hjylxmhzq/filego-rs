import classNames from 'classnames';
import path from 'path-browserify';
import { useEffect, useState } from 'react';
import { search_files } from '../../../apis/file';
import { FileIcon } from '../../../components/icon/icon';
import { useDebounceValue } from '../../../hooks/common';
import LoadingBar from './loading-bar';
import style from './search-input.module.less';

interface IProps {
  onClick?: (file: { name: string, dir: string, is_dir: boolean }) => void;
}

export default function SearchInput(props: IProps) {
  const [keyword, setKeyword] = useState('');
  const [debouncedKeyword, pending] = useDebounceValue(keyword);
  const [loading, setLoading] = useState(false);

  const [files, setFiles] = useState<any[]>([]);
  async function search(keyword: string) {
    setLoading(true);
    try {
      const files = await search_files(keyword);
      if (keyword) {
        setFiles(files);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  }
  useEffect(() => {
    if (!keyword) {
      setFiles([]);
      setLoading(false);
    }
  }, [keyword]);
  useEffect(() => {
    if (!debouncedKeyword) return;
    search(debouncedKeyword);
    // eslint-disable-next-line
  }, [debouncedKeyword]);

  function highlight(name: string, kw: string) {
    let segs = name.split(kw).map((v, idx) => {
      return <span key={idx}>{v}</span>
    });
    let newSegs = [];
    for (let i = 0; i < segs.length; i++) {
      newSegs.push(segs[i]);
      if (i !== segs.length - 1) {
        const kwEl = <span key={'h-' + i} className={style.highlight}>{kw}</span>
        newSegs.push(kwEl);
      }
    }
    return newSegs;
  }

  return <div className={style['container']} >
    <LoadingBar loading={(pending && !!keyword) || loading} />
    <input className={style['search']} type="text" placeholder='搜索' onChange={e => setKeyword(e.target.value)}></input>
    {
      !!keyword && <div tabIndex={0} className={classNames(style.list, 'scrollbar')}>
        {
          files.map((file) => {
            return <div onClick={() => {
              if (props.onClick) {
                props.onClick({ name: file.file_name, dir: path.dirname(file.file_path), is_dir: file.is_dir })
              }
            }} className={style.item} key={file.file_path}>
              <span className={style.left}>
                <FileIcon className={style['file-icon']} size={14} file={{ name: file.file_name, is_dir: file.is_dir }} />
                <span title={file.file_name} className={style.name}>{highlight(file.file_name, keyword)}</span>
              </span>
              <span className={style.right}>
                <span title={file.file_path} className={style.dir}>{file.file_path}</span>
              </span>
            </div>
          })
        }
        {(files.length === 0 && !pending && !loading && !!keyword) && <div className={style['no-result']}>无搜索结果</div>}
      </div>
    }
  </div>
}
