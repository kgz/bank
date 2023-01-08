import { useContext, useState } from 'react';
import styles from './login.module.scss'
import { ctx } from "../..";
import { Navigate } from 'react-router';
import Google from './google';
import Loader from '../../template/loading';
import fetch_api from '../../fetch/fetch';

const Login = () => {
    const {isLoggedin, setIsLoggedin} = useContext(ctx);

    const [active_sign, setActive_sign] = useState(true);
    // const isLoggedin = useContext(ctx);

    const [loading, setLoading] = useState(false);

    const [username, setUsername] = useState('user@userland.com');
    const [password, setPassword] = useState('1234');


    const login_post = async (e: any) => {
        e.preventDefault();
        setLoading(true);
        console.log(username)
        fetch_api('login?'
        + new URLSearchParams({
            username: username,
            pw: password,
        })
        ,
            {
                method: 'GET',
                headers: {
                    'Access-Control-Allow-Origin': 'http://localhost:3000/'
                }
            }

        )

        setTimeout(() => {
            setLoading(false);
            // setIsLoggedin(true);
        }, 2000);
    }

    return (
        (isLoggedin && <Navigate to="/" />) ||
        <div className={styles.login}>
            <div className={styles.login_card}>
                <div className={styles.login_sign_switch}>
                    <div className={styles.login_sign_switch_signin + ' ' + (active_sign ? styles.active : '')} onClick={()=>setActive_sign(true)}>SIGN IN</div>
                    <div className={styles.login_sign_switch_signup + ' ' + (!active_sign ? styles.active : '')} onClick={()=>setActive_sign(false)}>SIGN UP</div>
                    <div className={styles.login_sign_line + ' '  + (!active_sign ? styles.active : '')}/>
                </div>
                
                {loading && <Loader /> ||
                <div className={styles.login_sign + ' ' + (!active_sign ? styles.active : '')} >
                    <div className={styles.login_sign_signin}>
                        <form>
                            <input type="text" placeholder='email / username' onChange={(e:any)=>setUsername(e.target.value)} defaultValue={username}/>
                            <input type="password" placeholder='Password' onChange={(e:any)=>setPassword(e.target.value)} defaultValue={password}/>
                            <br/>
                            <button type="submit" onClick={login_post} className={styles.login__signInButton}>Sign In</button>

                            {/* <button className={styles.google_sign_in}></button> */}
                            {/* forgot */}
                            <div className={styles.login_sign_forgot}>
                                <a href="#">Forgot your password?</a>
                            </div>
                        </form>
        
                    </div>
                    <div className={styles.login_sign_signup }>
                        <h1>Sign Up</h1>
                        <form>
                            <div>
                            <input type="text" />
                            <h5>E-mail</h5>
                            <input type="text"  />
                            <h5>Password</h5>
                            <input type="password"/>
                            </div>
                            <div>

                            <button type="submit" onClick={() => setLoading(true)} className={styles.login__signInButton}>Sign Up</button>
                            </div>
                        </form>

                    </div>

                </div>}
               
               
               
                {/* <h1>Login</h1>
                <form>
                    <h5>E-mail</h5>
                    <input type="text" />
                    <h5>Password</h5>
                    <input type="password" />
                    <button type="submit" onClick={() => setIsLoggedin(true)} className={styles.login__signInButton}>Sign In</button>
                </form>
                <p>
                    By signing-in you agree to the AMAZON FAKE CLONE Conditions of Use & Sale. Please see our Privacy Notice, our Cookies Notice and our Interest-Based Ads Notice.
                </p>
                <button className={styles.login__registerButton}>Create your Amazon Account</button>

            */}
                </div> 
                

        </div>
    )

}

export default Login