import Header from "./header/header";
import styles from "./container.module.scss";
import { ctx } from "..";
import { useContext } from "react";
const Container = (props: any) => {
    const {isLoggedin, setIsLoggedin} = useContext(ctx);


    const { children, ...rest } = props;
    return (
        <span className={styles.main}>
            {isLoggedin &&  <Header /> }
            {children}
        </span>
    );
};

export default Container;