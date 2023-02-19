import { NavLink } from 'react-router-dom';
import styles from './header.module.scss';
import Image from '../../fetch/image';
import { userContext } from '../..';
import { useContext } from 'react';

const logo = require('./logo.png');

const Header = () => {

    const {user, setUser} = useContext(userContext);

    return (
        <header className={styles.header}>
            <div className={styles.header__logo}>
                <img src="/static/media/logo.png" alt="logo" />
            </div>
            {/* <div className={styles.header__search}>
                <input type="text" placeholder="Search" />
            </div> */}
            <div className={styles.header__profile}>
                <>{user.data?.username}</>
                <Image className={styles.icon} src="/me" key={0} alt={'Profile Image'} />
            </div>
        </header>

    )
}

export default Header
