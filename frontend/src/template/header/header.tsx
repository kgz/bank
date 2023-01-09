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
            <div className={styles.header__nav}>
                <div className={styles.header__nav__item}>
                    <NavLink to="/">Dashboard</NavLink>
                </div>
                <div className={styles.header__nav__item}>
                    <NavLink to="/migrations">Migrations</NavLink>
                </div>
                <div className={styles.header__nav__item}>
                    <NavLink to="/login">Login</NavLink>
                </div>
            </div>
        </header>

    )
}

export default Header