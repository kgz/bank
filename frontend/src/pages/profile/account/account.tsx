import { Form } from 'antd';
import { useEffect, useState } from 'react';
import styles from './account.module.scss';
import Cookies from 'universal-cookie';
import Imagea from '../../../fetch/image';

const Account = () => {
    const [update, setUpdate] = useState(0);
    const cookies = new Cookies();


    useEffect(() => {
        const controller = new AbortController();
        const signal = controller.signal;
        const token = cookies.get('token');

        const headers = {
            // 'Content-Type': 'image',
            'Content-Type': 'image/png',
            'Authorization': 'Bearer ' + token
        }







        return () => controller.abort();
    }, [update]);



    const [dummyData, setDummyData] = useState({
        name: "John Doe",
        email: "t@t.com",
        img: "http://127.0.0.1:3030/me",
        path: '512/727/727399.png'
    });

    const handleFile = (e: any) => {
        const file = e.target.files[0];
        const reader = new FileReader();
        reader.onload = () => {
            if (reader.readyState === 2) {
                console.log(reader.result);
                setDummyData({
                    ...dummyData,
                    img: reader.result as string,
                    path: file.name
                })
            }
        }
        reader.readAsDataURL(file);
    }


    return (
        <div className={styles.wrapper}>
            <div className={styles.account_logo}>
                <div className={styles.account_logo__img}>
                    {/* <img key={update} src = {"http://127.0.0.1:3030/me? " + new Date().getTime()} alt="profile" /> */}
                    <Imagea key={update} src={"http://127.0.0.1:3030/me"} alt="profile" />
                </div>
                <button onClick={()=>{setUpdate(update+1)}}>Update</button>
                <div className={styles.account_logo__name}>
                    <span className={styles.desc}>
                        Uplaod a photo that identifies you
                    </span>
                    <br/> 
                    <p>{dummyData.path}</p>
                </div>
                {/* uplaod button */}
                <div className={styles.account_logo__upload}>
                    <button onClick={()=>{
                        document.getElementById('file')?.click();
                    }}>Upload</button>
                </div>
                <input type="file" name="file" id="file" className={styles.inputfile}
                    onChange={handleFile}
                    accept="image/*"
                />
            </div>

        <div className={styles.account_form}> 

                <h1>Your Information</h1>
            <Form >
                <Form.Item label="Name" name="name">
                    <input type="text" placeholder={dummyData.name} />
                </Form.Item>
                <Form.Item label="Email" name="email">  
                    <input type="text" placeholder={dummyData.email} />
                </Form.Item>
                <Form.Item label="Password" name="password">
                    <input type="password" placeholder="********" />
                </Form.Item>
                <Form.Item label="Confirm Password" name="confirm_password">
                    <input type="password" placeholder="********" />
                </Form.Item>
                <Form.Item>
                    <button>Save</button>
                </Form.Item>
            </Form>
            </div>

        
        </div>

    );
};

export default Account;