import Header from "./header";
import styles from "./container.module.scss";
const Container = (props: any) => {
    const { children, ...rest } = props;
    return (
        <span className={styles.main}>
            {/* <Header /> */}
            {children}
        </span>
    );
};

export default Container;