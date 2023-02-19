import Header from "./header/header";
import styles from "./container.module.scss";
import { ctx } from "..";
import { useContext } from "react";
import Sidebar from "./header/sidebar";
const Container = (props: any) => {
    const {isLoggedin, setIsLoggedin} = useContext(ctx);


    const { children, ...rest } = props;
    return (
        <span className={styles.main}>
            {isLoggedin &&  <>
                <Header />
                <Sidebar/>
            </>
             }
            <div className={isLoggedin ? styles.main_content : ''}>
                {children}
            </div>
        </span>
    );
};

export default Container;