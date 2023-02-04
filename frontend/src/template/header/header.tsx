import { NavLink } from 'react-router-dom';
import styles from './header.module.scss';

const logo = require('./logo.png');

const Header = () => {
    return (
        <header className={styles.header}>
            <div className={styles.header__logo}>
                <img src={logo} alt="logo" />
            </div>
            {/* <div className={styles.header__search}>
                <input type="text" placeholder="Search" />
            </div> */}
            
        </header>

    )
}

export default Header
